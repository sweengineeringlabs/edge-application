//! `Application` trait — the boot gate for an edge application.

use std::future::Future;
use std::pin::Pin;

use crate::api::AppError;
use crate::api::NoopApplication;
use crate::api::app::dto::{ApplicationRunRequest, ApplicationRunResponse, NameRequest, NameResponse};

/// The top-level lifecycle contract for an edge application.
///
/// Implementors bring up all subsystems and hold the application open
/// until shutdown is signalled.
pub trait Application: Send + Sync {
    /// Stable identifier for this application.
    fn name(&self, _req: NameRequest) -> Result<NameResponse, AppError> {
        Ok(NameResponse { name: "application" })
    }

    /// Boot the application.
    ///
    /// Resolves when the application terminates or encounters a fatal error.
    fn run(
        &self,
        req: ApplicationRunRequest,
    ) -> Pin<Box<dyn Future<Output = Result<ApplicationRunResponse, AppError>> + Send + '_>>;

    /// Return a no-operation application for testing or default wiring.
    fn noop() -> NoopApplication
    where
        Self: Sized,
    {
        NoopApplication
    }
}
