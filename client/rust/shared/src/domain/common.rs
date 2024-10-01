//use std::error::Error;


// #[derive(Debug, PartialEq, thiserror::Error, uniffi::Error)]
// pub enum DomainError {
//     #[error("Division by zero is not allowed.")]
//     DivisionByZero,
// }

// pub trait DomainOperation<P, T> {
//     async fn perform(&self, arg: P) -> Result<T, Box<dyn Error>>;
// }

// pub trait DomainOperation<T> {
//     //fn perform(&self, callback: Box<dyn FnMut(Result<T, ()>) + Send + 'static>);
//     fn perform<C>(&self, callback: C)
//     where
//         C : Fn(dyn Result<T, Error> + Send);
// }
