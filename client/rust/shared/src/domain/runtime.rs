use log::error;
use std::{cell::OnceCell, collections::HashMap, future::Future, rc::Rc, sync::RwLock};
use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;
use wg::WaitGroup;

use crate::{
    data::DataServiceProvider,
    domain::{DomainError, DomainResult},
};

thread_local! {
    static STATE: OnceCell<Rc<DataServiceProvider>> = const { OnceCell::new() };
}

pub(crate) struct Runtime {
    pub(crate) pool: LocalPoolHandle,
    tasks: RwLock<HashMap<String, JoinHandle<()>>>,
}

impl Runtime {
    pub(crate) fn new(base_url: String, data_path: String) -> Result<Self, DomainError> {
        let pool = LocalPoolHandle::new(1);
        let wg = WaitGroup::new();
        let t_wg = wg.add(1);

        pool.spawn_pinned(async move || {
            let _ = DataServiceProvider::new(base_url, data_path).map(|provider| {
                STATE.with(|state| {
                    let _ = state.set(Rc::new(provider));
                });
            });
            t_wg.done();
        });
        wg.wait();

        Ok(Self {
            pool: pool,
            tasks: RwLock::new(HashMap::new()),
        })
    }

    pub(crate) async fn spawn<F, R>(
        &self,
        key: String,
        future: impl FnOnce(Rc<DataServiceProvider>) -> F + Send + 'static,
    ) -> DomainResult<R>
    where
        F: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        if let Some(task) = self.tasks.read().unwrap().get(&key) {
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

    #[allow(dead_code)]
    pub(crate) fn abort(&self) {
        for entry in self.tasks.read().unwrap().iter() {
            entry.1.abort();
        }
        self.tasks.write().unwrap().clear();
    }
}
