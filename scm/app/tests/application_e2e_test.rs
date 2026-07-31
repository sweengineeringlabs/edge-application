//! Integration tests — `Application` trait via SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{AppError, Application, ApplicationRunRequest, ApplicationRunResponse, NameRequest};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Echo;
impl Application for Echo {
    fn name(&self, _req: NameRequest) -> Result<edge_application_app::NameResponse, AppError> {
        Ok(edge_application_app::NameResponse { name: "echo" })
    }
    fn run(&self, _req: ApplicationRunRequest) -> BoxFuture<'_, Result<ApplicationRunResponse, AppError>> {
        Box::pin(async { Ok(ApplicationRunResponse) })
    }
}

struct AlwaysFails;
impl Application for AlwaysFails {
    fn run(&self, _req: ApplicationRunRequest) -> BoxFuture<'_, Result<ApplicationRunResponse, AppError>> {
        Box::pin(async { Err(AppError::BootFailed("deliberate".into())) })
    }
}

/// @covers: Application::name — configured name returned
#[test]
fn test_name_configured_value_returned_happy() {
    assert_eq!(Echo.name(NameRequest).unwrap().name, "echo");
}

/// @covers: Application::name — default impl when not overridden
#[test]
fn test_name_default_impl_returns_application_error() {
    assert_eq!(AlwaysFails.name(NameRequest).unwrap().name, "application");
}

/// @covers: Application::name — via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_returns_name_edge() {
    let app: &dyn Application = &Echo;
    assert_eq!(app.name(NameRequest).unwrap().name, "echo");
}

/// @covers: Application::run — success path
#[test]
fn test_run_returns_ok_happy() {
    assert_eq!(block_on(Echo.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
}

/// @covers: Application::run — failure propagates error
#[test]
fn test_run_returns_boot_failed_error() {
    let result = block_on(AlwaysFails.run(ApplicationRunRequest));
    assert!(result.is_err());
    let msg = match result {
        Err(e) => e.to_string(),
        Ok(_) => panic!("expected error"),
    };
    assert_eq!(msg, "boot failed: deliberate");
}

/// @covers: Application::run — repeated calls are independent
#[test]
fn test_run_repeated_calls_are_independent_edge() {
    assert_eq!(block_on(Echo.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
    assert_eq!(block_on(Echo.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
}

/// @covers: Application::noop — returns NoopApplication with default name
#[test]
fn test_noop_returns_noop_application_happy() {
    use edge_application_app::NoopApplication;
    let noop: NoopApplication = Echo::noop();
    assert_eq!(noop.name(NameRequest).unwrap().name, "application");
}

/// @covers: Application::noop — noop name differs from the caller's custom name
#[test]
fn test_noop_name_differs_from_caller_name_error() {
    use edge_application_app::NoopApplication;
    let noop: NoopApplication = Echo::noop();
    assert_ne!(noop.name(NameRequest).unwrap().name, Echo.name(NameRequest).unwrap().name);
}

/// @covers: Application::noop — returned NoopApplication is Copy
#[test]
fn test_noop_application_is_copy_edge() {
    use edge_application_app::NoopApplication;
    let noop: NoopApplication = Echo::noop();
    let copy = noop;
    assert_eq!(noop.name(NameRequest).unwrap().name, copy.name(NameRequest).unwrap().name);
}
