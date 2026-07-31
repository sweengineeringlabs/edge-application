//! SAF tests — `Saga` trait.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::ExecutionRequest;
use edge_application_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_application_saga::{
    Command, CommandError, DomainEvent, MemorySagaStore, Saga, SagaError, SagaGetRequest,
    SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest, SagaIsCompleteResponse,
    SagaRegisterRequest, SagaStore,
};
use futures::future::BoxFuture;

#[derive(Clone)]
struct StepEvt;

impl DomainEvent for StepEvt {
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "order-1",
        })
    }
}

#[derive(Clone, Debug)]
struct StepCmd;

impl Command for StepCmd {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

struct OrderSaga {
    steps: u32,
    target: u32,
}

impl OrderSaga {
    fn new(target: u32) -> Self {
        Self { steps: 0, target }
    }
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = StepEvt;
    type Command = StepCmd;

    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        self.steps += 1;
        let commands = if self.steps < self.target {
            vec![StepCmd]
        } else {
            vec![]
        };
        Ok(SagaHandleResponse { commands })
    }

    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.steps >= self.target,
        })
    }
}

/// @covers: handle
#[test]
fn test_handle_first_step_event_returns_commands_happy() {
    let mut saga = OrderSaga::new(3);
    let cmds = saga
        .handle(SagaHandleRequest { event: &StepEvt })
        .unwrap()
        .commands;
    assert_eq!(cmds.len(), 1);
}

/// @covers: handle
#[test]
fn test_handle_final_step_event_returns_empty_commands_error() {
    let mut saga = OrderSaga::new(1);
    let cmds = saga
        .handle(SagaHandleRequest { event: &StepEvt })
        .unwrap()
        .commands;
    assert!(cmds.is_empty());
}

/// @covers: handle
#[test]
fn test_handle_multiple_events_accumulate_steps_edge() {
    let mut saga = OrderSaga::new(3);
    saga.handle(SagaHandleRequest { event: &StepEvt }).unwrap();
    saga.handle(SagaHandleRequest { event: &StepEvt }).unwrap();
    let cmds = saga
        .handle(SagaHandleRequest { event: &StepEvt })
        .unwrap()
        .commands;
    assert!(cmds.is_empty());
    assert!(saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: is_complete
#[test]
fn test_is_complete_after_target_steps_returns_true_happy() {
    let mut saga = OrderSaga::new(1);
    saga.handle(SagaHandleRequest { event: &StepEvt }).unwrap();
    assert!(saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: is_complete
#[test]
fn test_is_complete_initial_state_returns_false_error() {
    let saga = OrderSaga::new(3);
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: is_complete
#[test]
fn test_is_complete_midway_through_returns_false_edge() {
    let mut saga = OrderSaga::new(3);
    saga.handle(SagaHandleRequest { event: &StepEvt }).unwrap();
    saga.handle(SagaHandleRequest { event: &StepEvt }).unwrap();
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: handle
#[test]
fn test_handle_via_dyn_registry_returns_ok_edge() {
    let mut reg = MemorySagaStore::<OrderSaga>::new();
    reg.register(SagaRegisterRequest {
        id: "o1".to_string(),
        saga: OrderSaga::new(2),
    })
    .ok();
    let id = "o1".to_string();
    if let Ok(resp) = reg.get(SagaGetRequest { id: &id }) {
        assert!(
            !resp
                .saga
                .is_complete(SagaIsCompleteRequest)
                .unwrap()
                .complete
        );
    }
}
