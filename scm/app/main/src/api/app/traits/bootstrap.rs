//! `Bootstrap` trait — produces [`Application`] instances from a wired service graph.

use crate::api::AppError;
use crate::api::NoopAppBootstrap;
use crate::api::NoopAppRuntime;
use crate::api::NoopAppSvcFactory;
use crate::api::app::dto::{ApplicationBuildRequest, ApplicationBuildResponse};

/// Constructs an [`Application`](crate::api::Application) from a resolved service graph.
///
/// Implementors wire all subsystems and return a ready-to-run application.
pub trait Bootstrap: Send + Sync {
    /// Build and return a ready-to-run [`Application`](crate::api::Application).
    fn build(&self, req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError>;

    /// Return a no-operation bootstrap for testing or default wiring.
    fn noop() -> NoopAppBootstrap
    where
        Self: Sized,
    {
        NoopAppBootstrap
    }

    /// Return a no-operation runtime for tests and default wiring.
    fn noop_runtime() -> NoopAppRuntime
    where
        Self: Sized,
    {
        NoopAppRuntime
    }

    /// Return the no-operation service factory for tests and structural scaffolding.
    fn noop_svc_factory() -> NoopAppSvcFactory
    where
        Self: Sized,
    {
        NoopAppSvcFactory
    }
}
