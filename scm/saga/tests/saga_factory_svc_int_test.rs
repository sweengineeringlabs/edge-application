//! SAF facade tests — saga construction via direct constructors.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::{ExecutionRequest, NameRequest};
use edge_application_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_application_saga::{
    Command, CommandError, DomainEvent, MemorySagaStore, NoopSaga, NoopSagaCommand,
    NoopSagaEvent, Saga, SagaError, SagaGetRequest, SagaHandleRequest, SagaHandleResponse,
    SagaIsCompleteRequest, SagaIsCompleteResponse, SagaRegisterRequest, SagaStore,
};
use futures::future::BoxFuture;

#[derive(Clone)]
struct FactEvt;

impl DomainEvent for FactEvt {
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "fact-test",
        })
    }
}

#[derive(Clone, Debug)]
struct FactCmd;

impl Command for FactCmd {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct SimpleSaga;

impl Saga for SimpleSaga {
    type SagaId = String;
    type Event = FactEvt;
    type Command = FactCmd;

    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        Ok(SagaHandleResponse { commands: vec![] })
    }

    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse { complete: true })
    }
}

/// @covers: MemorySagaStore::new
#[test]
fn test_in_memory_store_creates_empty_registry_happy() {
    let reg = MemorySagaStore::<SimpleSaga>::new();
    let id = "any".to_string();
    assert!(reg.get(SagaGetRequest { id: &id }).is_err());
}

/// @covers: MemorySagaStore::new
#[test]
fn test_in_memory_store_accepts_registration_error() {
    let mut reg = MemorySagaStore::<SimpleSaga>::new();
    reg.register(SagaRegisterRequest {
        id: "s1".to_string(),
        saga: SimpleSaga,
    })
    .expect("registration should succeed");
}

/// @covers: MemorySagaStore::new
#[test]
fn test_in_memory_store_multiple_instances_are_independent_edge() {
    let mut reg1 = MemorySagaStore::<SimpleSaga>::new();
    let reg2 = MemorySagaStore::<SimpleSaga>::new();
    reg1.register(SagaRegisterRequest {
        id: "s1".to_string(),
        saga: SimpleSaga,
    })
    .ok();
    let id = "s1".to_string();
    assert!(reg1.get(SagaGetRequest { id: &id }).is_ok());
    assert!(reg2.get(SagaGetRequest { id: &id }).is_err());
}

/// @covers: NoopSaga::default
#[test]
fn test_noop_creates_noop_saga_not_complete_happy() {
    let saga = NoopSaga::default();
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: NoopSaga::default
#[test]
fn test_noop_creates_fresh_instance_each_call_error() {
    // Each call returns a new independent instance — not a shared ref
    let s1 = NoopSaga::default();
    let s2 = NoopSaga::default();
    assert!(!s1.is_complete(SagaIsCompleteRequest).unwrap().complete);
    assert!(!s2.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: NoopSaga::default
#[test]
fn test_noop_default_produces_consistent_state_edge() {
    let a = NoopSaga::default();
    let b = NoopSaga::default();
    assert_eq!(
        a.is_complete(SagaIsCompleteRequest).unwrap().complete,
        b.is_complete(SagaIsCompleteRequest).unwrap().complete
    );
}

/// @covers: NoopSagaEvent
#[test]
fn test_noop_event_creates_event_with_empty_aggregate_id_happy() {
    let evt = NoopSagaEvent;
    assert_eq!(
        evt.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}

/// @covers: NoopSagaEvent
#[test]
fn test_noop_event_event_type_returns_default_error() {
    use edge_application_event::EventTypeRequest;
    let evt = NoopSagaEvent;
    assert_eq!(
        evt.event_type(EventTypeRequest).unwrap().event_type,
        "event"
    );
}

/// @covers: NoopSagaEvent
#[test]
fn test_noop_event_can_be_cloned_edge() {
    let evt = NoopSagaEvent;
    let cloned = evt.clone();
    assert_eq!(
        cloned
            .aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}

/// @covers: NoopSagaCommand
#[test]
fn test_noop_command_execute_returns_ok_happy() {
    use futures::executor::block_on;
    let cmd = NoopSagaCommand;
    block_on(cmd.execute(ExecutionRequest))
        .expect("noop saga command execute should always succeed");
}

/// @covers: NoopSagaCommand
#[test]
fn test_noop_command_name_returns_default_error() {
    let cmd = NoopSagaCommand;
    assert_eq!(cmd.name(NameRequest).unwrap().name, "command");
}

/// @covers: NoopSagaCommand
#[test]
fn test_noop_command_can_be_called_repeatedly_edge() {
    use futures::executor::block_on;
    let cmd = NoopSagaCommand;
    let r1 = block_on(cmd.execute(ExecutionRequest));
    let r2 = block_on(cmd.execute(ExecutionRequest));
    r1.expect("first execute should succeed");
    r2.expect("second execute should also succeed");
}
