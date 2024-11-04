use std::{collections::HashMap, future::Future};
use tokio::task::JoinHandle;

pub(crate) struct Runtime {
    tasks: HashMap<String, JoinHandle<()>>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub(crate) fn spawn<F>(&mut self, key: String, future: F)
    where 
        F: Future<Output = ()> + Send + 'static,
        F::Output: Send + 'static,
    {
        if let Some(task) = self.tasks.get(&key) {
            task.abort();
        }
        let x: JoinHandle<()> = tokio::task::spawn(future);
        self.tasks.insert(key, x);
    }

    pub(crate) fn abort(&mut self) {
        for entry in self.tasks.iter() {
            entry.1.abort();
        }
        self.tasks.clear();
    }
}
