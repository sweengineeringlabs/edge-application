//! Integration tests — `NoopApplication` (api noop type).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{
    AppError, Application, ApplicationRunRequest, ApplicationRunResponse, NameRequest, NoopApplication,
};
use futures::executor::block_on;

/// @covers: NoopApplication — default name is "application"
#[test]
fn test_noop_application_name_returns_application_happy() {
    assert_eq!(NoopApplication.name(NameRequest).unwrap().name, "application");
}

/// @covers: NoopApplication — run() never returns an error
#[test]
fn test_noop_application_run_never_errors_error() {
    let result: Result<ApplicationRunResponse, AppError> = block_on(NoopApplication.run(ApplicationRunRequest));
    assert_eq!(result, Ok(ApplicationRunResponse));
}

/// @covers: NoopApplication — is Copy; clone behaves identically
#[test]
fn test_noop_application_copy_preserves_name_edge() {
    let original = NoopApplication;
    let copy = original;
    assert_eq!(
        original.name(NameRequest).unwrap().name,
        copy.name(NameRequest).unwrap().name
    );
    assert_eq!(block_on(copy.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
}
