//! Integration tests — `AppServiceProvider` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{
    AppServiceProvider, ApplicationBuildRequest, NameRequest, NoopAppBootstrap, NoopAppSvcFactory,
    ProviderBuildRequest, ProviderBuildResponse,
};

struct PassthroughProvider(NoopAppBootstrap);

impl AppServiceProvider for PassthroughProvider {
    fn build(
        &self,
        _req: ProviderBuildRequest,
    ) -> Result<ProviderBuildResponse, edge_application_app::AppError> {
        Ok(ProviderBuildResponse {
            bootstrap: Box::new(self.0),
        })
    }
}

/// @covers: AppServiceProvider::build — built bootstrap resolves application successfully
#[test]
fn test_build_returns_bootstrap_that_builds_application_happy() {
    let provider = PassthroughProvider(NoopAppBootstrap);
    let bootstrap = provider.build(ProviderBuildRequest).unwrap().bootstrap;
    let app = bootstrap
        .build(ApplicationBuildRequest)
        .expect("bootstrap must succeed")
        .application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: AppServiceProvider::build — build always succeeds (no error path for noop)
#[test]
fn test_build_bootstrap_is_infallible_for_noop_error() {
    let provider = NoopAppSvcFactory;
    let bootstrap = provider.build(ProviderBuildRequest).unwrap().bootstrap;
    let app = bootstrap
        .build(ApplicationBuildRequest)
        .expect("noop provider bootstrap must build")
        .application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: AppServiceProvider::noop — factory method returns NoopAppSvcFactory
#[test]
fn test_noop_returns_noop_app_svc_factory_edge() {
    let f: NoopAppSvcFactory = NoopAppSvcFactory::noop();
    assert_eq!(format!("{f:?}"), "NoopAppSvcFactory");
}
