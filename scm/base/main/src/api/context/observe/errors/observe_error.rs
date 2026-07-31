//! `ObserveError` — errors produced by the observability factory.

/// Error produced when an observability backend cannot be initialised.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ObserveError {
    /// The requested backend is not available in the current environment.
    #[error("observe backend unavailable: {0}")]
    BackendUnavailable(String),
    /// The factory was called before the backend was initialised.
    #[error("observe backend not initialised")]
    NotInitialised,
}
