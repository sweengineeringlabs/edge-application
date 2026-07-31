//! Basic `Application` and `Bootstrap` usage example.

use edge_application_app::{
    AppError, Application, ApplicationBuildRequest, ApplicationBuildResponse, ApplicationRunRequest,
    ApplicationRunResponse, Bootstrap, NameRequest, NameResponse,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct GreetApp;

impl Application for GreetApp {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, AppError> {
        Ok(NameResponse { name: "greet" })
    }
    fn run(&self, _req: ApplicationRunRequest) -> BoxFuture<'_, Result<ApplicationRunResponse, AppError>> {
        Box::pin(async {
            println!("Hello from edge-domain-app!");
            Ok(ApplicationRunResponse)
        })
    }
}

struct GreetBootstrap;

impl Bootstrap for GreetBootstrap {
    fn build(&self, _req: ApplicationBuildRequest) -> Result<ApplicationBuildResponse, AppError> {
        Ok(ApplicationBuildResponse {
            application: Box::new(GreetApp),
        })
    }
}

fn main() -> Result<(), AppError> {
    let bootstrap = GreetBootstrap;
    let app = bootstrap.build(ApplicationBuildRequest)?.application;
    block_on(app.run(ApplicationRunRequest))?;
    Ok(())
}
