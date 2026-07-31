//! Layer-level coverage for `api/saga/dto/*.rs` request/response types.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_saga::{
    SagaGetRequest, SagaGetResponse, SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest,
    SagaIsCompleteResponse, SagaRegisterRequest,
};

/// @covers: SagaGetRequest
#[test]
fn test_saga_get_request_holds_id_happy() {
    let id = 42u32;
    let req = SagaGetRequest { id: &id };
    assert_eq!(*req.id, 42);
}

/// @covers: SagaGetResponse
#[test]
fn test_saga_get_response_holds_saga_happy() {
    let saga = "my-saga".to_string();
    let resp = SagaGetResponse { saga: &saga };
    assert_eq!(resp.saga, "my-saga");
}

/// @covers: SagaHandleRequest
#[test]
fn test_saga_handle_request_holds_event_happy() {
    let event = 7u32;
    let req = SagaHandleRequest { event: &event };
    assert_eq!(*req.event, 7);
}

/// @covers: SagaHandleResponse
#[test]
fn test_saga_handle_response_holds_commands_happy() {
    let resp = SagaHandleResponse {
        commands: vec![1, 2, 3],
    };
    assert_eq!(resp.commands, vec![1, 2, 3]);
}

/// @covers: SagaHandleResponse
#[test]
fn test_saga_handle_response_empty_commands_edge() {
    let resp: SagaHandleResponse<u32> = SagaHandleResponse { commands: vec![] };
    assert!(resp.commands.is_empty());
}

/// @covers: SagaIsCompleteRequest
#[test]
fn test_saga_is_complete_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SagaIsCompleteRequest>(), 0);
    let _ = SagaIsCompleteRequest;
}

/// @covers: SagaIsCompleteResponse
#[test]
fn test_saga_is_complete_response_true_happy() {
    let resp = SagaIsCompleteResponse { complete: true };
    assert!(resp.complete);
}

/// @covers: SagaIsCompleteResponse
#[test]
fn test_saga_is_complete_response_false_error() {
    let resp = SagaIsCompleteResponse { complete: false };
    assert!(!resp.complete);
}

/// @covers: SagaRegisterRequest
#[test]
fn test_saga_register_request_holds_id_and_saga_happy() {
    let req = SagaRegisterRequest {
        id: 1u32,
        saga: "value".to_string(),
    };
    assert_eq!(req.id, 1);
    assert_eq!(req.saga, "value");
}
