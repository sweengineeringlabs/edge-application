//! Error type for the application layer.

/// Errors produced by [`Application`](crate::api::Application) and [`Bootstrap`](crate::api::Bootstrap).
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AppError {
    /// Application boot sequence failed.
    #[error("boot failed: {0}")]
    BootFailed(String),
    /// Service construction failed.
    #[error("service creation failed: {0}")]
    CreationFailed(String),
}
