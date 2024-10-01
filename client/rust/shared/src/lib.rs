use std::sync::Arc;
use tokio::sync::Mutex;

pub mod domain;
mod data;

uniffi::setup_scaffolding!();

pub type ArcMutex<T> = Arc<Mutex<T>>;

#[inline(always)]
pub fn arc_mutex<T>(value: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(value))
}
