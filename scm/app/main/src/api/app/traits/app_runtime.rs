//! [`AppRuntime`] — the mandatory boot gate for an edge application.

use std::future::Future;
use std::pin::Pin;

use crate::api::app::dto::{NameRequest, NameResponse, RuntimeBootRequest, RuntimeBootResponse};
use crate::api::AppError;
use crate::api::NoopAppRuntime;

/// Enforces the canonical boot sequence: `Bootstrap::build` → `Application::run`.
///
/// Every edge application MUST enter through this gate.  Callers may not
/// construct an [`Application`](crate::api::Application) and call `run` directly;
/// they must go through an `AppRuntime` implementation so the boot sequence is
/// auditable and replaceable without changing call sites.
pub trait AppRuntime: Send + Sync {
    /// Stable identifier for this runtime.
    fn name(&self, _req: NameRequest) -> Result<NameResponse, AppError> {
        Ok(NameResponse {
            name: "app_runtime",
        })
    }

    /// Execute the full boot gate: build an application via `bootstrap`, then run it.
    ///
    /// Propagates any error from `Bootstrap::build` or `Application::run`.
    fn boot<'a>(
        &'a self,
        req: RuntimeBootRequest<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<RuntimeBootResponse, AppError>> + Send + 'a>>;

    /// Return the no-operation runtime for tests and default wiring.
    fn noop() -> NoopAppRuntime
    where
        Self: Sized,
    {
        NoopAppRuntime
    }
}
