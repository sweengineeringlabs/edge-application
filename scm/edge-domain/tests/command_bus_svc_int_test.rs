#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — CommandBus is exported from the crate root.
#![cfg(feature = "command")]

use edge_application::Command;
use edge_application::CommandBus;
use edge_application::CommandError;
use edge_application::DirectCommandBusRequest;
use edge_application::Domain;
use edge_application::DomainRuntime;
use edge_application_command::{
    CommandDispatchRequest, ExecutionRequest, NameRequest, NameResponse,
};

struct Noop;
impl Command for Noop {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "noop".to_string(),
        })
    }
    fn execute(
        &self,
        _req: ExecutionRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test]
async fn test_command_bus_svc_facade_dispatch_ok_command() {
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(Noop)
        })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_command_bus_svc_facade_dispatch_failing_command() {
    struct Bad;
    impl Command for Bad {
        fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
            Ok(NameResponse {
                name: "bad".to_string(),
            })
        }
        fn execute(
            &self,
            _req: ExecutionRequest,
        ) -> futures::future::BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Err(CommandError::InvalidInput("rejected".into())) })
        }
    }
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(Bad)
        })
        .await
        .is_err());
}
