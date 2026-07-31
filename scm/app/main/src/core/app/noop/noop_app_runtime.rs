use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::AppRuntime;
use crate::api::NoopAppRuntime;
use crate::api::RuntimeBootRequest;
use crate::api::RuntimeBootResponse;

impl AppRuntime for NoopAppRuntime {
    fn boot<'a>(
        &'a self,
        _req: RuntimeBootRequest<'a>,
    ) -> BoxFuture<'a, Result<RuntimeBootResponse, AppError>> {
        Box::pin(async { Ok(RuntimeBootResponse) })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::api::{ApplicationBuildRequest, ApplicationBuildResponse, Bootstrap, NameRequest};

    struct NoopAppRuntimeStubBootstrap;
    impl Bootstrap for NoopAppRuntimeStubBootstrap {
        fn build(&self, _req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError> {
            Err(AppError::BootFailed("stub".into()))
        }
    }

    #[test]
    fn test_boot_noop_always_ok_happy() {
        let r = NoopAppRuntime;
        let b = NoopAppRuntimeStubBootstrap;
        assert_eq!(
            block_on(r.boot(RuntimeBootRequest { bootstrap: &b })),
            Ok(RuntimeBootResponse)
        );
    }

    #[test]
    fn test_boot_noop_ignores_bootstrap_error_error() {
        let r = NoopAppRuntime;
        let b = NoopAppRuntimeStubBootstrap;
        assert_eq!(
            block_on(r.boot(RuntimeBootRequest { bootstrap: &b })),
            Ok(RuntimeBootResponse)
        );
    }

    #[test]
    fn test_name_returns_app_runtime_edge() {
        assert_eq!(NoopAppRuntime.name(NameRequest).unwrap().name, "app_runtime");
    }
}
