//! Integration tests — `NoopAppSvcFactory` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{
    AppServiceProvider, ApplicationBuildRequest, NameRequest, NoopAppSvcFactory, ProviderBuildRequest,
};

/// @covers: NoopAppSvcFactory — build returns a working bootstrap
#[test]
fn test_noop_app_svc_factory_build_returns_bootstrap_happy() {
    let factory = NoopAppSvcFactory;
    let app = factory
        .build(ProviderBuildRequest)
        .unwrap()
        .bootstrap
        .build(ApplicationBuildRequest)
        .expect("NoopAppSvcFactory must produce a buildable bootstrap")
        .application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: NoopAppSvcFactory — name is stable
#[test]
fn test_noop_app_svc_factory_name_is_stable_error() {
    let factory = NoopAppSvcFactory;
    assert_eq!(factory.name(NameRequest).unwrap().name, "app_service_provider");
}

/// @covers: NoopAppSvcFactory — is Copy
#[test]
fn test_noop_app_svc_factory_is_copy_edge() {
    let a = NoopAppSvcFactory;
    let b = a;
    assert_eq!(a, b);
}
