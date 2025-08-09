use std::{collections::HashMap, future::Future};
use tokio::task::JoinHandle;

pub(crate) struct Runtime {
    tasks: HashMap<String, JoinHandle<()>>,
}

lazy_static::lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("LingoLessons Thread")
        .build().unwrap();
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self { tasks: HashMap::new() }
    }

    pub(crate) fn spawn<F>(&mut self, key: String, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
        F::Output: Send + 'static,
    {
        if let Some(task) = self.tasks.get(&key) {
            task.abort();
        }
        let x: JoinHandle<()> = RUNTIME.spawn(future);
        self.tasks.insert(key, x);
    }

    #[allow(dead_code)]
    pub(crate) fn abort(&mut self) {
        for entry in self.tasks.iter() {
            entry.1.abort();
        }
        self.tasks.clear();
    }
}
