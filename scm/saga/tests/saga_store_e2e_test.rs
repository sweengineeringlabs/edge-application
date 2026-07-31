//! SAF tests — `SagaStore` trait via `MemorySagaStore`.
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
struct RegEvt;

impl DomainEvent for RegEvt {
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "reg-test",
        })
    }
}

#[derive(Clone, Debug)]
struct RegCmd;

impl Command for RegCmd {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct PingSaga {
    done: bool,
}

impl Saga for PingSaga {
    type SagaId = String;
    type Event = RegEvt;
    type Command = RegCmd;

    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
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

fn new_store() -> MemorySagaStore<PingSaga> {
    MemorySagaStore::new()
}

/// @covers: register
#[test]
fn test_register_new_saga_returns_ok_happy() {
    let mut store = new_store();
    store
        .register(SagaRegisterRequest {
            id: "s1".to_string(),
            saga: PingSaga::default(),
        })
        .expect("registration should succeed");
}

/// @covers: register
#[test]
fn test_register_duplicate_id_returns_already_registered_error() {
    let mut store = new_store();
    store
        .register(SagaRegisterRequest {
            id: "s1".to_string(),
            saga: PingSaga::default(),
        })
        .ok();
    let err = match store.register(SagaRegisterRequest {
        id: "s1".to_string(),
        saga: PingSaga::default(),
    }) {
        Err(e) => e,
        Ok(()) => panic!("expected Err"),
    };
    assert_eq!(err, SagaError::AlreadyRegistered("s1".to_string()));
}

/// @covers: register
#[test]
fn test_register_multiple_sagas_with_different_ids_edge() {
    let mut store = new_store();
    store
        .register(SagaRegisterRequest {
            id: "a".to_string(),
            saga: PingSaga::default(),
        })
        .expect("register a");
    store
        .register(SagaRegisterRequest {
            id: "b".to_string(),
            saga: PingSaga::default(),
        })
        .expect("register b");
    let a = "a".to_string();
    let b = "b".to_string();
    assert!(store.get(SagaGetRequest { id: &a }).is_ok());
    assert!(store.get(SagaGetRequest { id: &b }).is_ok());
}

/// @covers: get
#[test]
fn test_get_registered_saga_returns_saga_happy() {
    let mut store = new_store();
    store
        .register(SagaRegisterRequest {
            id: "s1".to_string(),
            saga: PingSaga::default(),
        })
        .ok();
    let id = "s1".to_string();
    let saga = store.get(SagaGetRequest { id: &id });
    assert!(saga.is_ok());
    if let Ok(resp) = saga {
        assert!(
            !resp
                .saga
                .is_complete(SagaIsCompleteRequest)
                .unwrap()
                .complete
        );
    }
}

/// @covers: get
#[test]
fn test_get_unknown_id_returns_not_found_error() {
    let store = new_store();
    let id = "ghost".to_string();
    let err = match store.get(SagaGetRequest { id: &id }) {
        Err(e) => e,
        Ok(_) => panic!("expected Err"),
    };
    assert_eq!(err, SagaError::NotFound("ghost".to_string()));
}

/// @covers: get
#[test]
fn test_get_after_register_is_consistent_edge() {
    let mut store = new_store();
    store
        .register(SagaRegisterRequest {
            id: "id1".to_string(),
            saga: PingSaga::default(),
        })
        .ok();
    let id = "id1".to_string();
    let saga = match store.get(SagaGetRequest { id: &id }) {
        Ok(resp) => resp.saga,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}
