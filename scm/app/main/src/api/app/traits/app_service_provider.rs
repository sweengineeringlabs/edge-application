//! [`AppServiceProvider`] — composition-root contract for wiring service dependencies into a [`Bootstrap`].

use crate::api::app::dto::{
    NameRequest, NameResponse, ProviderBuildRequest, ProviderBuildResponse,
};
use crate::api::AppError;
use crate::api::NoopAppSvcFactory;

/// Port contract for the composition root that wires the service graph and produces a [`Bootstrap`].
///
/// Implementors create and wire all service dependencies needed to build an
/// [`Application`](crate::api::Application).  The returned Bootstrap then assembles those
/// services into a runnable Application.
///
/// The canonical flow is:
/// ```text
/// AppServiceProvider::build() → Bootstrap::build() → Application::run()
/// ```
pub trait AppServiceProvider: Send + Sync {
    /// Stable identifier for this provider.
    fn name(&self, _req: NameRequest) -> Result<NameResponse, AppError> {
        Ok(NameResponse {
            name: "app_service_provider",
        })
    }

    /// Build a configured [`Bootstrap`](crate::api::Bootstrap) from the wired service graph.
    fn build(&self, req: ProviderBuildRequest) -> Result<ProviderBuildResponse, AppError>;

    /// Return the no-operation provider for tests and structural scaffolding.
    fn noop() -> NoopAppSvcFactory
    where
        Self: Sized,
    {
        NoopAppSvcFactory
    }
}
