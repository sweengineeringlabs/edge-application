//! Integration tests for `MemoryRegistry` — covers the types/ file directly.
// @allow: no_mocks_in_integration — MemoryRegistry is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_registry::{
    EmptinessRequest, MemoryRegistry, LenRequest, RegisterRequest, Registry,
    RegistryLookupRequest,
};

/// @covers: MemoryRegistry::new — starts empty
#[test]
fn test_new_starts_empty_happy() {
    let r: MemoryRegistry<str> = MemoryRegistry::new();
    assert!(r.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: MemoryRegistry (Default) — equivalent to new, starts empty
#[test]
fn test_default_starts_empty_error() {
    let r: MemoryRegistry<str> = MemoryRegistry::default();
    assert_eq!(r.len(LenRequest).unwrap().count, 0);
}

/// @covers: MemoryRegistry — round-trips an unsized (`str`) entry
#[test]
fn test_round_trip_unsized_entry_edge() {
    let r: MemoryRegistry<str> = MemoryRegistry::new();
    r.register(RegisterRequest {
        id: "k".to_string(),
        entry: Arc::from("v"),
    })
    .unwrap();
    let entry = r
        .get(RegistryLookupRequest {
            id: "k".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.as_deref(), Some("v"));
}
