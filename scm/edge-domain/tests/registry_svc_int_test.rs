//! Umbrella-level integration tests that exercise `edge-domain-registry` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
// @allow: no_mocks_in_integration — MemoryRegistry is a reference implementation in the public API, not a mock.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_registry::{
    DeregisterRequest, ListIdsRequest, MemoryRegistry, RegisterRequest, Registry,
    RegistryLookupRequest,
};
use std::sync::Arc;

/// @covers: Registry::register, Registry::get
#[test]
fn test_in_memory_registry_register_and_get_happy() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    registry
        .register(RegisterRequest {
            id: "key1".to_string(),
            entry: Arc::new(42),
        })
        .unwrap();
    let entry = registry
        .get(RegistryLookupRequest {
            id: "key1".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.map(|v| *v), Some(42));
}

/// @covers: Registry::get (nonexistent key)
#[test]
fn test_in_memory_registry_get_nonexistent_key_returns_none_happy() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    let entry = registry
        .get(RegistryLookupRequest {
            id: "missing".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry, None);
}

/// @covers: Registry::register (duplicate key)
#[test]
fn test_in_memory_registry_register_duplicate_key_overwrites_happy() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    registry
        .register(RegisterRequest {
            id: "key1".to_string(),
            entry: Arc::new(42),
        })
        .unwrap();
    registry
        .register(RegisterRequest {
            id: "key1".to_string(),
            entry: Arc::new(100),
        })
        .unwrap();
    let entry = registry
        .get(RegistryLookupRequest {
            id: "key1".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.map(|v| *v), Some(100));
}

/// @covers: Registry::deregister
#[test]
fn test_in_memory_registry_deregister_removes_entry_happy() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    registry
        .register(RegisterRequest {
            id: "key1".to_string(),
            entry: Arc::new(42),
        })
        .unwrap();
    let removed = registry
        .deregister(DeregisterRequest {
            id: "key1".to_string(),
        })
        .unwrap()
        .was_present;
    assert!(removed);
    let entry = registry
        .get(RegistryLookupRequest {
            id: "key1".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry, None);
}

/// @covers: Registry::list_ids
#[test]
fn test_in_memory_registry_list_ids_returns_all_registered_keys_happy() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    registry
        .register(RegisterRequest {
            id: "key1".to_string(),
            entry: Arc::new(42),
        })
        .unwrap();
    registry
        .register(RegisterRequest {
            id: "key2".to_string(),
            entry: Arc::new(100),
        })
        .unwrap();

    let keys = registry.list_ids(ListIdsRequest).unwrap().ids;
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
}

/// @covers: MemoryRegistry construction
#[test]
fn test_in_memory_registry_new_creates_empty_registry_happy() {
    let registry: MemoryRegistry<String> = MemoryRegistry::new();
    let keys = registry.list_ids(ListIdsRequest).unwrap().ids;
    assert!(keys.is_empty());
}

/// @covers: Registry::register (multiple items)
#[test]
fn test_in_memory_registry_register_multiple_items_happy() {
    let registry: MemoryRegistry<String> = MemoryRegistry::new();
    for i in 0..10 {
        registry
            .register(RegisterRequest {
                id: format!("key{}", i),
                entry: Arc::new(format!("value_{}", i)),
            })
            .unwrap();
    }

    let keys = registry.list_ids(ListIdsRequest).unwrap().ids;
    assert_eq!(keys.len(), 10);
}

/// @covers: MemoryRegistry construction (register + get round-trip)
#[test]
fn test_registry_factory_creates_registry_happy() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    registry
        .register(RegisterRequest {
            id: "key1".to_string(),
            entry: Arc::new(42),
        })
        .unwrap();
    let entry = registry
        .get(RegistryLookupRequest {
            id: "key1".to_string(),
        })
        .unwrap()
        .entry;
    assert_eq!(entry.map(|v| *v), Some(42));
}

/// @covers: Registry::deregister (nonexistent key)
#[test]
fn test_in_memory_registry_deregister_nonexistent_key_returns_false_edge() {
    let registry: MemoryRegistry<i32> = MemoryRegistry::new();
    let removed = registry
        .deregister(DeregisterRequest {
            id: "missing".to_string(),
        })
        .unwrap()
        .was_present;
    assert!(!removed);
}

/// @covers: Registry::list_ids (empty state)
#[test]
fn test_in_memory_registry_list_ids_empty_returns_no_items_edge() {
    let registry: MemoryRegistry<String> = MemoryRegistry::new();
    let keys = registry.list_ids(ListIdsRequest).unwrap().ids;
    assert!(keys.is_empty());
}
