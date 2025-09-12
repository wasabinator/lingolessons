use std::{collections::HashMap, future::Future, sync::RwLock};
use tokio::task::JoinHandle;

pub(crate) struct Runtime {
    tasks: RwLock<HashMap<String, JoinHandle<()>>>,
}

lazy_static::lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("LingoLessons Thread")
        .build().unwrap();
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self { tasks: RwLock::new(HashMap::new()) }
    }

    pub(crate) fn spawn<F>(&self, key: String, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
        F::Output: Send + 'static,
    {
        if let Some(task) = self.tasks.read().unwrap().get(&key) {
            task.abort();
        }

        let x: JoinHandle<()> = RUNTIME.spawn(future);
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
