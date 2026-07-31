//! Layer-level coverage for the small request/response value types declared under
//! `api/registry/dto/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape or field values.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_registry::{
    DeregisterRequest, DeregisterResponse, EmptinessRequest, EmptinessResponse, LenRequest,
    LenResponse, ListIdsRequest, ListIdsResponse, RegisterRequest, RegisterResponse,
    RegistryLookupRequest, RegistryLookupResponse, TryRegisterRequest, TryRegisterResponse,
};

// --- zero-sized marker request types ---

/// @covers: EmptinessRequest
#[test]
fn test_emptiness_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EmptinessRequest>(), 0);
    let _ = EmptinessRequest;
}

/// @covers: LenRequest
#[test]
fn test_len_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<LenRequest>(), 0);
    let _ = LenRequest;
}

/// @covers: ListIdsRequest
#[test]
fn test_list_ids_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ListIdsRequest>(), 0);
    let _ = ListIdsRequest;
}

/// @covers: RegisterResponse
#[test]
fn test_register_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<RegisterResponse>(), 0);
    let _ = RegisterResponse;
}

/// @covers: TryRegisterResponse
#[test]
fn test_try_register_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<TryRegisterResponse>(), 0);
    let _ = TryRegisterResponse;
}

// --- field-carrying request/response types ---

/// @covers: DeregisterRequest
#[test]
fn test_deregister_request_holds_id_happy() {
    let r = DeregisterRequest {
        id: "a".to_string(),
    };
    assert_eq!(r.id, "a");
}

/// @covers: DeregisterResponse
#[test]
fn test_deregister_response_holds_was_present_error() {
    let r = DeregisterResponse { was_present: false };
    assert!(!r.was_present);
}

/// @covers: EmptinessResponse
#[test]
fn test_emptiness_response_holds_empty_flag_happy() {
    let r = EmptinessResponse { empty: true };
    assert!(r.empty);
}

/// @covers: LenResponse
#[test]
fn test_len_response_holds_count_happy() {
    let r = LenResponse { count: 5 };
    assert_eq!(r.count, 5);
}

/// @covers: ListIdsResponse
#[test]
fn test_list_ids_response_holds_ids_happy() {
    let r = ListIdsResponse {
        ids: vec!["a".to_string(), "b".to_string()],
    };
    assert_eq!(r.ids, vec!["a".to_string(), "b".to_string()]);
}

/// @covers: RegisterRequest
#[test]
fn test_register_request_holds_id_and_entry_happy() {
    let r: RegisterRequest<str> = RegisterRequest {
        id: "a".to_string(),
        entry: Arc::from("alpha"),
    };
    assert_eq!(r.id, "a");
    assert_eq!(&*r.entry, "alpha");
}

/// @covers: TryRegisterRequest
#[test]
fn test_try_register_request_holds_id_and_entry_happy() {
    let r: TryRegisterRequest<str> = TryRegisterRequest {
        id: "a".to_string(),
        entry: Arc::from("alpha"),
    };
    assert_eq!(r.id, "a");
    assert_eq!(&*r.entry, "alpha");
}

/// @covers: RegistryLookupRequest
#[test]
fn test_registry_lookup_request_holds_id_happy() {
    let r = RegistryLookupRequest {
        id: "a".to_string(),
    };
    assert_eq!(r.id, "a");
}

/// @covers: RegistryLookupResponse
#[test]
fn test_registry_lookup_response_holds_none_when_absent_edge() {
    let r: RegistryLookupResponse<str> = RegistryLookupResponse { entry: None };
    assert!(r.entry.is_none());
}

/// @covers: RegistryLookupResponse
#[test]
fn test_registry_lookup_response_holds_some_entry_happy() {
    let entry: Arc<str> = Arc::from("alpha");
    let r = RegistryLookupResponse { entry: Some(entry) };
    assert_eq!(r.entry.as_deref(), Some("alpha"));
}
