//! Integration tests — `NoopAppBootstrap` (api noop type).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{ApplicationBuildRequest, Bootstrap, NameRequest, NoopAppBootstrap};

/// @covers: NoopAppBootstrap — build() returns a NoopApplication
#[test]
fn test_noop_app_bootstrap_build_returns_application_happy() {
    let result = NoopAppBootstrap.build(ApplicationBuildRequest);
    let app = result.expect("noop bootstrap should build").application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: NoopAppBootstrap — build() never returns Err
#[test]
fn test_noop_app_bootstrap_build_never_errors_error() {
    let result = NoopAppBootstrap.build(ApplicationBuildRequest);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().application.name(NameRequest).unwrap().name, "application");
}

/// @covers: NoopAppBootstrap — is Copy; repeated builds all succeed
#[test]
fn test_noop_app_bootstrap_is_copy_repeated_builds_ok_edge() {
    let b = NoopAppBootstrap;
    let copy = b;
    for _ in 0..3 {
        let app = copy
            .build(ApplicationBuildRequest)
            .expect("each build should succeed")
            .application;
        assert_eq!(app.name(NameRequest).unwrap().name, "application");
    }
}
