//! Basic `Command` usage example.

use edge_application_command::{
    Command, CommandDispatchRequest, CommandError, DirectCommandBus, ExecutionRequest,
    NameRequest, NameResponse,
};
use futures::future::BoxFuture;

struct Ping;
impl Command for Ping {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "ping".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::main]
async fn main() {
    let bus = DirectCommandBus;
    let result = edge_application_command::CommandBus::dispatch(
        &bus,
        CommandDispatchRequest {
            command: Box::new(Ping),
        },
    )
    .await;
    println!("{:?}", result);
}
