//! Integration tests — `NoopAppRuntime` type.

use edge_application_app::{
    AppError, AppRuntime, ApplicationBuildRequest, ApplicationBuildResponse, Bootstrap,
    NoopAppBootstrap, NoopAppRuntime, RuntimeBootRequest, RuntimeBootResponse,
};
use futures::executor::block_on;

struct FailBootstrap;

impl Bootstrap for FailBootstrap {
    fn build(&self, _req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError> {
        Err(AppError::BootFailed("forced".into()))
    }
}

/// @covers: NoopAppRuntime — boot always returns Ok regardless of bootstrap
#[test]
fn test_noop_app_runtime_boot_always_ok_happy() {
    let r = NoopAppRuntime;
    assert_eq!(
        block_on(r.boot(RuntimeBootRequest {
            bootstrap: &NoopAppBootstrap
        })),
        Ok(RuntimeBootResponse)
    );
}

/// @covers: NoopAppRuntime — boot ignores bootstrap failures
#[test]
fn test_noop_app_runtime_ignores_bootstrap_failure_error() {
    let r = NoopAppRuntime;
    assert_eq!(
        block_on(r.boot(RuntimeBootRequest {
            bootstrap: &FailBootstrap
        })),
        Ok(RuntimeBootResponse)
    );
}

/// @covers: NoopAppRuntime — is Copy; two copies both work
#[test]
fn test_noop_app_runtime_is_copy_edge() {
    let a = NoopAppRuntime;
    let b = a;
    assert_eq!(a, b);
}
