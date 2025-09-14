use domain::DomainError;
use std::sync::Arc;
use tokio::task::JoinError;

pub mod common;
mod data;
pub mod domain;

uniffi::setup_scaffolding!();

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
