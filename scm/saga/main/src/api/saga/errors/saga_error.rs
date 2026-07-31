use thiserror::Error;

/// Errors that can occur when registering or looking up sagas.
///
/// These cover registry bookkeeping only — [`Saga::handle`](crate::Saga::handle)
/// is infallible and stages commands rather than returning errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SagaError {
    /// A saga is already registered under the requested id.
    #[error("a saga is already registered under id '{0}'")]
    AlreadyRegistered(String),

    /// No saga is registered under the requested id.
    #[error("no saga is registered under id '{0}'")]
    NotFound(String),

    /// A command emitted by a saga failed during dispatch.
    #[error("saga command dispatch failed: {0}")]
    CommandDispatchFailed(String),
}
