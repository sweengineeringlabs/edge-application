use crate::api::AppError;
use crate::api::ApplicationBuildRequest;
use crate::api::ApplicationBuildResponse;
use crate::api::Bootstrap;
use crate::api::NoopAppBootstrap;
use crate::api::NoopApplication;

impl Bootstrap for NoopAppBootstrap {
    fn build(&self, _req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError> {
        Ok(ApplicationBuildResponse {
            application: Box::new(NoopApplication),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::NameRequest;

    #[test]
    fn test_build_returns_application_happy() {
        let result = NoopAppBootstrap.build(ApplicationBuildRequest);
        let app = result.expect("build should succeed").application;
        assert_eq!(app.name(NameRequest).unwrap().name, "application");
    }

    #[test]
    fn test_build_never_returns_err_error() {
        let result = NoopAppBootstrap.build(ApplicationBuildRequest);
        assert!(result.is_ok());
        let app = result.unwrap().application;
        assert_eq!(app.name(NameRequest).unwrap().name, "application");
    }

    #[test]
    fn test_build_multiple_times_each_ok_edge() {
        for _ in 0..3 {
            let app = NoopAppBootstrap
                .build(ApplicationBuildRequest)
                .expect("build should succeed")
                .application;
            assert_eq!(app.name(NameRequest).unwrap().name, "application");
        }
    }
}
