use std::{future::Future, sync::{Arc}};
use domain::DomainError;
use tokio::{sync::{Mutex, MutexGuard}, task::{JoinError, LocalSet}};
use lazy_static::lazy_static;

pub mod domain;
mod data;

uniffi::setup_scaffolding!();

pub type ArcMutex<T> = Arc<Mutex<T>>;

#[inline(always)]
pub fn arc_mutex<T>(value: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(value))
}

// #[inline(always)]
// pub fn shared<T>(value: T) -> Shared<T> {
//     Rc::new(RefCell::new(value))
// }

//type Shared<T> = Rc<RefCell<T>>;

type DateTime = chrono::DateTime<chrono::Local>;
uniffi::custom_type!(DateTime, std::time::SystemTime);

impl UniffiCustomTypeConverter for chrono::DateTime<chrono::Local> {
    type Builtin = std::time::SystemTime;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Self::from(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

impl From<JoinError> for DomainError {
    #[inline]
    fn from(value: JoinError) -> Self {
        DomainError::Unexpected(format!("Thread join error: {:?}", value))
    }
}

lazy_static! {
    // Global Tokio runtime
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_name("LingoLessons thread")
        .build()
        .expect("Failed to build tokio runtime");
}

pub(crate) trait SpawnLocal {
    async fn spawn_domain<T>(&self, future: impl Future<Output = T>) -> T;
}

impl SpawnLocal for tokio::runtime::Runtime {
    async fn spawn_domain<T>(&self, future: impl Future<Output = T>) -> T {
        //self.block_on(
        //    async move {
        let local = LocalSet::new();
        local.run_until(future).await
        //    }
        //)
    }
}
