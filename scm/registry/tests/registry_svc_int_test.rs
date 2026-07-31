//! Integration tests — `Registry` trait via `MemoryRegistry`.
// @allow: no_mocks_in_integration — MemoryRegistry is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_registry::{
    DeregisterRequest, EmptinessRequest, MemoryRegistry, LenRequest, ListIdsRequest,
    RegisterRequest, Registry, RegistryLookupRequest,
};

fn make_reg() -> MemoryRegistry<str> {
    MemoryRegistry::default()
}

/// @covers: Registry::register — handler is retrievable after registration
#[test]
fn test_register_entry_can_be_retrieved_happy() {
    let reg = make_reg();
    reg.register(RegisterRequest {
        id: "alpha".to_string(),
        entry: Arc::from("a"),
    })
    .unwrap();
    let entry = reg
        .get(RegistryLookupRequest {
            id: "alpha".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.as_deref(), Some("a"));
}

/// @covers: Registry::deregister — existing entry
#[test]
fn test_deregister_existing_entry_returns_true_happy() {
    let reg = make_reg();
    reg.register(RegisterRequest {
        id: "beta".to_string(),
        entry: Arc::from("b"),
    })
    .unwrap();
    assert!(
        reg.deregister(DeregisterRequest {
            id: "beta".to_string()
        })
        .unwrap()
        .was_present
    );
}

/// @covers: Registry::get — missing id
#[test]
fn test_get_missing_id_returns_none_error() {
    let reg = make_reg();
    assert!(reg
        .get(RegistryLookupRequest {
            id: "missing".to_string()
        })
        .unwrap()
        .entry
        .is_none());
}

/// @covers: Registry::list_ids
#[test]
fn test_list_ids_returns_registered_ids_happy() {
    let reg = make_reg();
    reg.register(RegisterRequest {
        id: "b".to_string(),
        entry: Arc::from("b"),
    })
    .unwrap();
    reg.register(RegisterRequest {
        id: "a".to_string(),
        entry: Arc::from("a"),
    })
    .unwrap();
    let mut ids = reg.list_ids(ListIdsRequest).unwrap().ids;
    ids.sort();
    assert_eq!(ids, vec!["a", "b"]);
}

/// @covers: Registry::len
#[test]
fn test_len_reflects_current_count_happy() {
    let reg = make_reg();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    reg.register(RegisterRequest {
        id: "x".to_string(),
        entry: Arc::from("x"),
    })
    .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

/// @covers: Registry::is_empty
#[test]
fn test_is_empty_new_registry_returns_true_edge() {
    let reg = make_reg();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}
