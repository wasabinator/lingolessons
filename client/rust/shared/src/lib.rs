use std::sync::Arc;

use domain::DomainError;
use tokio::{sync::Mutex, task::JoinError};

pub mod domain;
mod data;

uniffi::setup_scaffolding!();

pub type ArcMutex<T> = Arc<Mutex<T>>;

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
