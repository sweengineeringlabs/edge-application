//! `AppRuntime` impl for [`LocalAppRuntime`] — executes Bootstrap → Application → run.

use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::AppRuntime;
use crate::api::ApplicationBuildRequest;
use crate::api::ApplicationRunRequest;
use crate::api::LocalAppRuntime;
use crate::api::RuntimeBootRequest;
use crate::api::RuntimeBootResponse;

impl AppRuntime for LocalAppRuntime {
    fn boot<'a>(
        &'a self,
        req: RuntimeBootRequest<'a>,
    ) -> BoxFuture<'a, Result<RuntimeBootResponse, AppError>> {
        Box::pin(async move {
            let app = req.bootstrap.build(ApplicationBuildRequest)?.application;
            app.run(ApplicationRunRequest).await?;
            Ok(RuntimeBootResponse)
        })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::api::{AppRuntime, NameRequest, NoopAppBootstrap};

    #[test]
    fn test_boot_ok_bootstrap_completes_happy() {
        assert_eq!(
            block_on(LocalAppRuntime.boot(RuntimeBootRequest {
                bootstrap: &NoopAppBootstrap
            })),
            Ok(RuntimeBootResponse)
        );
    }

    #[test]
    fn test_boot_called_twice_both_complete_error() {
        let r = LocalAppRuntime;
        assert_eq!(
            block_on(r.boot(RuntimeBootRequest {
                bootstrap: &NoopAppBootstrap
            })),
            Ok(RuntimeBootResponse)
        );
        assert_eq!(
            block_on(r.boot(RuntimeBootRequest {
                bootstrap: &NoopAppBootstrap
            })),
            Ok(RuntimeBootResponse)
        );
    }

    #[test]
    fn test_name_returns_app_runtime_edge() {
        assert_eq!(
            LocalAppRuntime.name(NameRequest).unwrap().name,
            "app_runtime"
        );
    }
}
