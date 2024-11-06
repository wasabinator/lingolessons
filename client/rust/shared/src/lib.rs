use std::sync::Arc;

use domain::DomainError;
use tokio::{sync::{Mutex, MutexGuard}, task::JoinError};

pub mod domain;
mod data;

uniffi::setup_scaffolding!();

pub type ArcMutex<T> = Arc<Mutex<T>>;

/// Trait that will allow an operation to be perform during a lock. It Makes it very clear what the duration of the lock is.
/// The lock will be dropped on the function return since it goes out of scope.
pub trait Run<T, U> 
where T: Send {
    fn run<F>(self, op: F) -> impl std::future::Future<Output = U> + Send
    where 
        F: FnOnce(&mut MutexGuard<'_, T>) -> U + Send;

    //fn run(self, op: fn(&mut MutexGuard<'_, T>) -> U) -> impl std::future::Future<Output = U> + Send;
}

impl<T, U> Run<T, U> for &Arc<Mutex<T>>
where T: Send {
    async fn run<F>(self, op: F) -> U
    where 
        F: FnOnce(&mut MutexGuard<'_, T>) -> U + Send {
        let arc= self.clone();
        let mut locked = arc.lock().await;
        op(&mut locked)
    }
}

#[inline(always)]
pub fn arc_mutex<T>(value: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(value))
}

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
