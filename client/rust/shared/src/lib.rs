use domain::DomainError;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, MutexGuard, OwnedMutexGuard},
    task::JoinError,
};

pub mod common;
mod data;
pub mod domain;

uniffi::setup_scaffolding!();

pub type ArcMutex<T> = Arc<Mutex<T>>;

/// Trait that will allow an operation to be perform during a lock. It Makes it very clear what the duration of the lock is.
/// The lock will be dropped on the function return since it goes out of scope.
pub trait Run<T, U>
where
    T: Send,
{
    fn run<F>(self, op: F) -> impl std::future::Future<Output = U> + Send
    where
        F: FnOnce(&mut MutexGuard<'_, T>) -> U + Send;

    fn launch<F, Fut>(self, op: F) -> impl std::future::Future<Output = U> + Send + Sync
    where
        F: FnOnce(OwnedMutexGuard<T>) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = U> + Send + Sync;
}

impl<T, U> Run<T, U> for &Arc<Mutex<T>>
where
    T: Send,
{
    async fn run<F>(self, op: F) -> U
    where
        F: FnOnce(&mut MutexGuard<'_, T>) -> U + Send,
    {
        let arc = self.clone();
        let mut guard = arc.lock().await;
        op(&mut guard)
    }

    async fn launch<F, Fut>(self, op: F) -> U
    where
        F: FnOnce(OwnedMutexGuard<T>) -> Fut,
        Fut: std::future::Future<Output = U> + Send + Sync,
    {
        let guard = self.clone().lock_owned().await;
        op(guard).await
    }
}

// pub async fn run<T1, T2, F>(a: T1, b: T2, op: F) -> U
// where
//     F: FnOnce(&mut ArcMutex<T1>, &mut ArcMutex<T2>) -> U + Send,
//     T1: Clone + Send,
//     T2: Send + Clone,
// {
//     let guard1 = a.clone().lock();
//     let guard2 = b.clone().lock();
//     op(&mut guard1, &mut guard2)
// }

pub async fn launch<T1, T2, U, F, Fut>(a: &mut ArcMutex<T1>, b: &mut ArcMutex<T2>, op: F) -> U
where
    F: FnOnce(&mut T1, &mut T2) -> Fut,
    Fut: std::future::Future<Output = U> + Send + Sync,
    T1: Send,
    T2: Send,
{
    let mut guard1 = a.clone().lock_owned().await;
    let mut guard2 = b.clone().lock_owned().await;
    op(&mut guard1, &mut guard2).await
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
        DomainError::Unexpected(format!("Thread join error: {value:?}"))
    }
}
