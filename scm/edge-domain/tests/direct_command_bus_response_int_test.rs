//! Integration tests for `DirectCommandBusResponse`.
#![cfg(feature = "command")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Command, CommandBus, CommandError, DirectCommandBusRequest, Domain, DomainRuntime,
};
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

/// @covers: DirectCommandBusResponse
#[test]
fn test_direct_command_bus_response_bus_field_dispatches_happy() {
    futures::executor::block_on(async {
        let resp = Domain.direct_command_bus(DirectCommandBusRequest).unwrap();
        let bus: &dyn CommandBus = resp.bus.as_ref();
        assert!(bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(Noop)
            })
            .await
            .is_ok());
    });
}

/// @covers: DirectCommandBusResponse
#[test]
fn test_direct_command_bus_response_bus_field_propagates_command_error() {
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
    futures::executor::block_on(async {
        let resp = Domain.direct_command_bus(DirectCommandBusRequest).unwrap();
        assert!(resp
            .bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(Bad)
            })
            .await
            .is_err());
    });
}

/// @covers: DirectCommandBusResponse
#[test]
fn test_direct_command_bus_response_bus_is_uniquely_owned_edge() {
    let resp = Domain.direct_command_bus(DirectCommandBusRequest).unwrap();
    assert_eq!(std::sync::Arc::strong_count(&resp.bus), 1);
}
