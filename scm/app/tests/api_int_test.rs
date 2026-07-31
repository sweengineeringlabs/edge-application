//! Layer-level coverage for the small request/response value types declared under
//! `api/app/dto/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape or field values.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{
    ApplicationBuildRequest, ApplicationBuildResponse, ApplicationRunRequest, ApplicationRunResponse,
    NameRequest, NameResponse, NoopApplication, ProviderBuildRequest, ProviderBuildResponse,
    RuntimeBootRequest, RuntimeBootResponse,
};

/// @covers: ApplicationBuildRequest
#[test]
fn test_application_build_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ApplicationBuildRequest>(), 0);
    let _ = ApplicationBuildRequest;
}

/// @covers: ApplicationBuildResponse
#[test]
fn test_application_build_response_holds_application_happy() {
    let r = ApplicationBuildResponse {
        application: Box::new(NoopApplication),
    };
    assert_eq!(r.application.name(NameRequest).unwrap().name, "application");
}

/// @covers: ApplicationRunRequest
#[test]
fn test_application_run_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ApplicationRunRequest>(), 0);
    let _ = ApplicationRunRequest;
}

/// @covers: ApplicationRunResponse
#[test]
fn test_application_run_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ApplicationRunResponse>(), 0);
    let _ = ApplicationRunResponse;
}

/// @covers: NameRequest
#[test]
fn test_name_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NameRequest>(), 0);
    let _ = NameRequest;
}

/// @covers: NameResponse
#[test]
fn test_name_response_holds_name_happy() {
    let r = NameResponse { name: "svc" };
    assert_eq!(r.name, "svc");
}

/// @covers: ProviderBuildRequest
#[test]
fn test_provider_build_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ProviderBuildRequest>(), 0);
    let _ = ProviderBuildRequest;
}

/// @covers: ProviderBuildResponse
#[test]
fn test_provider_build_response_holds_bootstrap_happy() {
    use edge_application_app::NoopAppBootstrap;
    let r = ProviderBuildResponse {
        bootstrap: Box::new(NoopAppBootstrap),
    };
    let app = r
        .bootstrap
        .build(ApplicationBuildRequest)
        .unwrap()
        .application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: RuntimeBootRequest
#[test]
fn test_runtime_boot_request_holds_bootstrap_happy() {
    use edge_application_app::NoopAppBootstrap;
    let b = NoopAppBootstrap;
    let r = RuntimeBootRequest { bootstrap: &b };
    let app = r.bootstrap.build(ApplicationBuildRequest).unwrap().application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: RuntimeBootResponse
#[test]
fn test_runtime_boot_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<RuntimeBootResponse>(), 0);
    let _ = RuntimeBootResponse;
}
