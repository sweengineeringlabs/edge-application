//! `AppServiceProvider` impl for [`LocalAppServiceProvider`] — returns a `NoopAppBootstrap` as the default service graph.

use crate::api::AppError;
use crate::api::AppServiceProvider;
use crate::api::LocalAppServiceProvider;
use crate::api::NoopAppBootstrap;
use crate::api::ProviderBuildRequest;
use crate::api::ProviderBuildResponse;

impl AppServiceProvider for LocalAppServiceProvider {
    fn build(&self, _req: ProviderBuildRequest) -> Result<ProviderBuildResponse, AppError> {
        Ok(ProviderBuildResponse {
            bootstrap: Box::new(NoopAppBootstrap),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{ApplicationBuildRequest, NameRequest};

    #[test]
    fn test_build_returns_bootstrap_happy() {
        let f = LocalAppServiceProvider;
        let app = f
            .build(ProviderBuildRequest)
            .expect("LocalAppServiceProvider must produce a buildable bootstrap")
            .bootstrap
            .build(ApplicationBuildRequest)
            .expect("bootstrap must build")
            .application;
        assert_eq!(app.name(NameRequest).unwrap().name, "application");
    }

    #[test]
    fn test_build_called_twice_both_succeed_error() {
        let f = LocalAppServiceProvider;
        let app1 = f
            .build(ProviderBuildRequest)
            .expect("first build must succeed")
            .bootstrap
            .build(ApplicationBuildRequest)
            .expect("first bootstrap build must succeed")
            .application;
        let app2 = f
            .build(ProviderBuildRequest)
            .expect("second build must succeed")
            .bootstrap
            .build(ApplicationBuildRequest)
            .expect("second bootstrap build must succeed")
            .application;
        assert_eq!(
            app1.name(NameRequest).unwrap().name,
            app2.name(NameRequest).unwrap().name
        );
    }

    #[test]
    fn test_name_returns_app_service_provider_edge() {
        assert_eq!(
            LocalAppServiceProvider.name(NameRequest).unwrap().name,
            "app_service_provider"
        );
    }
}
