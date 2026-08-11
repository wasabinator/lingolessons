use crate::UniffiCustomTypeConverter;
use std::sync::Mutex;
use std::{cell::RefCell, str::FromStr, sync::Arc};
use tokio_util::task::LocalPoolHandle;
use uuid::Uuid;

// Use `Uuid` as a custom type, with `String` as the Builtin
uniffi::custom_type!(Uuid, String);

impl UniffiCustomTypeConverter for Uuid {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Uuid::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

/// Generic broadcaster which publishes to a keyed observer
/// This assumes one subscriber per published class of key
pub(crate) struct Broadcaster<O: ?Sized> {
    observer: RefCell<Option<Arc<O>>>,
}

impl<O: ?Sized> Broadcaster<O> {
    pub(crate) fn new() -> Self {
        Self {
            observer: RefCell::new(None),
        }
    }

    pub(crate) fn subscribe(&self, observer: Arc<O>) {
        self.observer.borrow_mut().replace(observer); // replaces any existing subscriber
    }

    pub(crate) fn unsubscribe(&self) {
        self.observer.borrow_mut().take();
    }

    pub(crate) fn notify(&self, f: impl FnOnce(&Arc<O>)) {
        if let Some(o) = self.observer.borrow().as_ref() {
            f(o);
        }
    }
}

#[derive(uniffi::Object)]
pub struct Subscription {
    pool: LocalPoolHandle,
    unsubscribe: Mutex<Option<Box<dyn FnOnce() + Send>>>, // Send-only closure, run once
}

impl std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("unsubscribe", &"<closure>")
            .finish()
    }
}

impl Subscription {
    pub(crate) fn new(pool: LocalPoolHandle, unsubscribe: impl FnOnce() + Send + 'static) -> Self {
        Self {
            pool,
            unsubscribe: Mutex::new(Some(Box::new(unsubscribe))),
        }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(unsubscribe) = self.unsubscribe.lock().unwrap().take() {
            self.pool.spawn_pinned(move || async move {
                unsubscribe();
            });
        }
    }
}
