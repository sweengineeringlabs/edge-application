use crate::api::AppError;
use crate::api::AppServiceProvider;
use crate::api::NoopAppBootstrap;
use crate::api::NoopAppSvcFactory;
use crate::api::ProviderBuildRequest;
use crate::api::ProviderBuildResponse;

impl AppServiceProvider for NoopAppSvcFactory {
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
    fn test_build_returns_noop_bootstrap_happy() {
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

    #[test]
    fn test_build_noop_bootstrap_application_name_is_default_error() {
        let factory = NoopAppSvcFactory;
        let bootstrap = factory.build(ProviderBuildRequest).unwrap().bootstrap;
        let app = bootstrap
            .build(ApplicationBuildRequest)
            .expect("build succeeds")
            .application;
        assert_eq!(app.name(NameRequest).unwrap().name, "application");
    }

    #[test]
    fn test_name_returns_app_service_provider_edge() {
        assert_eq!(
            NoopAppSvcFactory.name(NameRequest).unwrap().name,
            "app_service_provider"
        );
    }
}
