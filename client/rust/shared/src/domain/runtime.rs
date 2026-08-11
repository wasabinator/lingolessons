use log::error;
use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    future::Future,
    rc::Rc,
};
use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;
use wg::WaitGroup;

use crate::{
    data::DataServiceProvider,
    domain::{auth::start_session_publish_loop, DomainError, DomainResult},
};

thread_local! {
    static STATE: OnceCell<Rc<DataServiceProvider>> = const { OnceCell::new() };
    /// Detached background jobs (e.g. repo syncs) keyed so a new job aborts a stale one with
    /// the same key. Only touched from the single runtime worker thread.
    static TASKS: RefCell<HashMap<String, JoinHandle<()>>> = RefCell::new(HashMap::new());
}

/// Single threaded runtime. All jobs run pinned to one worker thread which owns the
/// `DataServiceProvider` (via thread local `STATE`), so shared state is `Rc` and needs no locks.
pub(crate) struct Runtime {
    pub(crate) pool: LocalPoolHandle,
}

impl Runtime {
    pub(crate) fn new(base_url: String, data_path: String) -> Result<Self, DomainError> {
        let pool = LocalPoolHandle::new(1);
        let wg = WaitGroup::new();
        let t_wg = wg.add(1);

        let pool_clone = pool.clone();
        pool.spawn_pinned(async move || {
            let _ = DataServiceProvider::new(base_url, data_path, pool_clone).map(|provider| {
                let provider = Rc::new(provider);
                STATE.with(|state| {
                    let _ = state.set(provider.clone());
                });
                start_session_publish_loop(&provider);
            });
            t_wg.done();
        });
        wg.wait();

        Ok(Self { pool })
    }

    /// Run a job on the runtime thread and await its result. Jobs must be `Send` as they
    /// cross into the runtime worker thread.
    pub(crate) async fn spawn<F, R>(
        &self,
        key: String,
        future: impl FnOnce(Rc<DataServiceProvider>) -> F + Send + 'static,
    ) -> DomainResult<R>
    where
        F: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        if let Some(task) = TASKS.with(|t| t.borrow_mut().remove(&key)) {
            task.abort();
        }

        let handle = self.pool.spawn_pinned(move || async move {
            let state = STATE
                .with(|state| state.get().cloned())
                .expect("state initialised in Runtime::new() before any job");
            future(state).await
        });

        handle.await.map_err(|e| {
            error!("Runtime: job '{key}' failed {e:?}");
            DomainError::Unexpected("Runtime: job '{key}' failed {e:?}".into())
        })
    }

    /// Fire and forget a background job keyed by `key`, replacing any job with the same key.
    /// Must be called from the runtime worker thread (repositories are only ever invoked
    /// there), which lets the job capture `Rc` state without a `Send` bound.
    pub(crate) fn spawn_detached<F>(
        key: String,
        future: impl FnOnce(Rc<DataServiceProvider>) -> F + 'static,
    ) where
        F: Future<Output = ()> + 'static,
    {
        if let Some(task) = TASKS.with(|t| t.borrow_mut().remove(&key)) {
            task.abort();
        }

        let handle = tokio::task::spawn_local(async move {
            let state = STATE
                .with(|state| state.get().cloned())
                .expect("state initialised in Runtime::new() before any job");
            future(state).await
        });
        TASKS.with(|t| {
            t.borrow_mut().insert(key, handle);
        });
    }

    /// Abort all tracked background jobs.
    pub(crate) fn abort() {
        let tasks = TASKS.with(|t| t.borrow_mut().drain().collect::<Vec<_>>());
        for (_, task) in tasks {
            task.abort();
        }
    }
}
