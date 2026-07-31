use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::Application;
use crate::api::ApplicationRunRequest;
use crate::api::ApplicationRunResponse;
use crate::api::NoopApplication;

impl Application for NoopApplication {
    fn run(&self, _req: ApplicationRunRequest) -> BoxFuture<'_, Result<ApplicationRunResponse, AppError>> {
        Box::pin(async { Ok(ApplicationRunResponse) })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::api::NameRequest;

    #[test]
    fn test_run_noop_happy() {
        let app = NoopApplication;
        assert_eq!(block_on(app.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
    }

    #[test]
    fn test_run_noop_twice_both_ok_error() {
        let app = NoopApplication;
        assert_eq!(block_on(app.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
        assert_eq!(block_on(app.run(ApplicationRunRequest)), Ok(ApplicationRunResponse));
    }

    #[test]
    fn test_name_default_is_application_edge() {
        assert_eq!(NoopApplication.name(NameRequest).unwrap().name, "application");
    }
}
