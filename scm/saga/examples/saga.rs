//! Basic `Saga` usage example.

use edge_application_command::{Command, CommandError, ExecutionRequest};
use edge_application_event::DomainEvent;
use edge_application_saga::{
    Saga, SagaError, SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest,
    SagaIsCompleteResponse,
};
use futures::future::BoxFuture;

struct Signal;
impl DomainEvent for Signal {}
impl Command for Signal {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct MySaga {
    done: bool,
}

impl Saga for MySaga {
    type SagaId = String;
    type Event = Signal;
    type Command = Signal;
    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Signal>,
    ) -> Result<SagaHandleResponse<Signal>, SagaError> {
        self.done = true;
        Ok(SagaHandleResponse { commands: vec![] })
    }
    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.done,
        })
    }
}

fn main() {
    let mut s = MySaga { done: false };
    if let Err(e) = s.handle(SagaHandleRequest { event: &Signal }) {
        eprintln!("handle failed: {e}");
        return;
    }
    match s.is_complete(SagaIsCompleteRequest) {
        Ok(resp) => println!("complete: {}", resp.complete),
        Err(e) => eprintln!("is_complete failed: {e}"),
    }
}
