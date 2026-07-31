//! Integration tests for `MemorySagaStore`.
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
struct RegEvt2;

impl DomainEvent for RegEvt2 {
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "reg2",
        })
    }
}

#[derive(Clone, Debug)]
struct RegCmd2;

impl Command for RegCmd2 {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Default)]
struct CountingSaga {
    count: u32,
}

impl Saga for CountingSaga {
    type SagaId = String;
    type Event = RegEvt2;
    type Command = RegCmd2;

    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        self.count += 1;
        Ok(SagaHandleResponse { commands: vec![] })
    }

    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.count >= 3,
        })
    }
}

#[test]
fn test_new_store_is_empty_happy() {
    let store = MemorySagaStore::<CountingSaga>::new();
    let id = "x".to_string();
    assert!(store.get(SagaGetRequest { id: &id }).is_err());
}

#[test]
fn test_default_store_is_empty_error() {
    let store = MemorySagaStore::<CountingSaga>::default();
    let id = "x".to_string();
    assert!(store.get(SagaGetRequest { id: &id }).is_err());
}

#[test]
fn test_store_stores_and_retrieves_saga_edge() {
    let mut store = MemorySagaStore::<CountingSaga>::new();
    store
        .register(SagaRegisterRequest {
            id: "c1".to_string(),
            saga: CountingSaga::default(),
        })
        .ok();
    let id = "c1".to_string();
    let saga = match store.get(SagaGetRequest { id: &id }) {
        Ok(resp) => resp.saga,
        Err(e) => panic!("expected saga, got: {e}"),
    };
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}
