use std::{cell::OnceCell, collections::HashMap, future::Future, rc::Rc, sync::RwLock};
use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;
use wg::WaitGroup;

use crate::{data::DataServiceProvider, domain::DomainError};

thread_local! {
    static STATE: OnceCell<Rc<DataServiceProvider>> = const { OnceCell::new() };
}

pub(crate) struct Runtime {
    pool: LocalPoolHandle,
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

    pub(crate) fn spawn<F>(&self, key: String, future: impl FnOnce() -> F + Send + 'static)
    where
        F: Future<Output = ()> + 'static,
    {
        if let Some(task) = self.tasks.read().unwrap().get(&key) {
            task.abort();
        }

        let x: JoinHandle<()> = self.pool.spawn_pinned(future);
        self.tasks.write().unwrap().insert(key, x);
    }

    #[allow(dead_code)]
    pub(crate) fn abort(&self) {
        for entry in self.tasks.read().unwrap().iter() {
            entry.1.abort();
        }
        self.tasks.write().unwrap().clear();
    }
}
