//! Integration tests — `Bootstrap` trait via SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_app::{
    AppError, Application, ApplicationBuildRequest, ApplicationBuildResponse, ApplicationRunRequest,
    ApplicationRunResponse, Bootstrap, NameRequest, NameResponse,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct EchoApp;
impl Application for EchoApp {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, AppError> {
        Ok(NameResponse { name: "echo" })
    }
    fn run(&self, _req: ApplicationRunRequest) -> BoxFuture<'_, Result<ApplicationRunResponse, AppError>> {
        Box::pin(async { Ok(ApplicationRunResponse) })
    }
}

struct AlwaysBuilds;
impl Bootstrap for AlwaysBuilds {
    fn build(&self, _req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError> {
        Ok(ApplicationBuildResponse {
            application: Box::new(EchoApp),
        })
    }
}

struct NeverBuilds;
impl Bootstrap for NeverBuilds {
    fn build(&self, _req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError> {
        Err(AppError::CreationFailed("not wired".into()))
    }
}

/// @covers: Bootstrap::build — success path
#[test]
fn test_build_returns_application_happy() {
    let result = AlwaysBuilds.build(ApplicationBuildRequest);
    assert!(result.is_ok());
    let app = result.unwrap().application;
    assert_eq!(app.name(NameRequest).unwrap().name, "echo");
}

/// @covers: Bootstrap::build — failure propagates error
#[test]
fn test_build_fails_with_creation_error_error() {
    let result = NeverBuilds.build(ApplicationBuildRequest);
    assert!(result.is_err());
    let msg = match result {
        Err(e) => e.to_string(),
        Ok(_) => panic!("expected error"),
    };
    assert_eq!(msg, "service creation failed: not wired");
}

/// @covers: Bootstrap::build — produced application is runnable
#[test]
fn test_build_application_runs_successfully_edge() {
    let app = AlwaysBuilds.build(ApplicationBuildRequest).unwrap().application;
    assert_eq!(block_on(app.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
}

/// @covers: Bootstrap::noop — returns NoopAppBootstrap that builds successfully
#[test]
fn test_noop_returns_noop_bootstrap_happy() {
    use edge_application_app::NoopAppBootstrap;
    let noop: NoopAppBootstrap = AlwaysBuilds::noop();
    let app = noop
        .build(ApplicationBuildRequest)
        .expect("noop bootstrap should build")
        .application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: Bootstrap::noop — built application name differs from caller's custom app
#[test]
fn test_noop_bootstrap_build_returns_default_name_error() {
    use edge_application_app::NoopAppBootstrap;
    let noop: NoopAppBootstrap = AlwaysBuilds::noop();
    let app = noop.build(ApplicationBuildRequest).expect("build succeeds").application;
    assert_ne!(app.name(NameRequest).unwrap().name, "echo");
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: Bootstrap::noop — NoopAppBootstrap is Copy
#[test]
fn test_noop_bootstrap_is_copy_edge() {
    use edge_application_app::NoopAppBootstrap;
    let noop: NoopAppBootstrap = AlwaysBuilds::noop();
    let copy = noop;
    let app = copy.build(ApplicationBuildRequest).expect("copy builds ok").application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: Bootstrap::noop_runtime — returns NoopAppRuntime
#[test]
fn test_noop_runtime_returns_noop_app_runtime_happy() {
    use edge_application_app::NoopAppRuntime;
    let r: NoopAppRuntime = AlwaysBuilds::noop_runtime();
    assert_eq!(format!("{r:?}"), "NoopAppRuntime");
}

/// @covers: Bootstrap::noop_runtime — returned value has the expected name
#[test]
fn test_noop_runtime_name_is_app_runtime_error() {
    use edge_application_app::{AppRuntime, NoopAppRuntime};
    let r: NoopAppRuntime = AlwaysBuilds::noop_runtime();
    assert_eq!(r.name(NameRequest).unwrap().name, "app_runtime");
}

/// @covers: Bootstrap::noop_runtime — is Copy
#[test]
fn test_noop_runtime_is_copy_edge() {
    use edge_application_app::NoopAppRuntime;
    let a: NoopAppRuntime = AlwaysBuilds::noop_runtime();
    let b = a;
    assert_eq!(a, b);
}

/// @covers: Bootstrap::noop_svc_factory — returns NoopAppSvcFactory
#[test]
fn test_noop_svc_factory_returns_noop_app_svc_factory_happy() {
    use edge_application_app::NoopAppSvcFactory;
    let f: NoopAppSvcFactory = AlwaysBuilds::noop_svc_factory();
    assert_eq!(format!("{f:?}"), "NoopAppSvcFactory");
}

/// @covers: Bootstrap::noop_svc_factory — built bootstrap produces a runnable application
#[test]
fn test_noop_svc_factory_build_always_ok_error() {
    use edge_application_app::{AppServiceProvider, NoopAppSvcFactory, ProviderBuildRequest};
    let f: NoopAppSvcFactory = AlwaysBuilds::noop_svc_factory();
    let app = f
        .build(ProviderBuildRequest)
        .unwrap()
        .bootstrap
        .build(ApplicationBuildRequest)
        .expect("noop provider bootstrap must build")
        .application;
    assert_eq!(app.name(NameRequest).unwrap().name, "application");
}

/// @covers: Bootstrap::noop_svc_factory — is Copy
#[test]
fn test_noop_svc_factory_is_copy_edge() {
    use edge_application_app::NoopAppSvcFactory;
    let a: NoopAppSvcFactory = AlwaysBuilds::noop_svc_factory();
    let b = a;
    assert_eq!(a, b);
}
