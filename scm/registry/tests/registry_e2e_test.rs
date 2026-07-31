//! SAF facade tests — `Registry` trait via `MemoryRegistry`.
// @allow: no_mocks_in_integration — MemoryRegistry is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_registry::{
    DeregisterRequest, EmptinessRequest, MemoryRegistry, LenRequest, ListIdsRequest,
    RegisterRequest, Registry, RegistryError, RegistryLookupRequest, TryRegisterRequest,
};

fn reg() -> MemoryRegistry<str> {
    MemoryRegistry::new()
}

fn register(r: &MemoryRegistry<str>, id: &str, entry: &str) {
    r.register(RegisterRequest {
        id: id.to_string(),
        entry: Arc::from(entry),
    })
    .unwrap();
}

fn get(r: &MemoryRegistry<str>, id: &str) -> Option<Arc<str>> {
    r.get(RegistryLookupRequest {
        id: id.to_string(),
    })
    .unwrap()
    .entry
}

// ── register ──────────────────────────────────────────────────────────────────
/// @covers: Registry::register
#[test]
fn test_register_then_get_returns_entry_happy() {
    let r = reg();
    register(&r, "a", "alpha");
    assert_eq!(get(&r, "a").as_deref(), Some("alpha"));
}

/// @covers: Registry::register
#[test]
fn test_register_does_not_create_unrelated_entries_error() {
    let r = reg();
    register(&r, "a", "alpha");
    assert!(get(&r, "b").is_none());
}

/// @covers: Registry::register
#[test]
fn test_register_same_id_twice_replaces_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    register(&r, "a", "beta");
    assert_eq!(get(&r, "a").as_deref(), Some("beta"));
    assert_eq!(r.len(LenRequest).unwrap().count, 1);
}

// ── try_register ──────────────────────────────────────────────────────────────
/// @covers: Registry::try_register
#[test]
fn test_try_register_new_id_succeeds_happy() {
    let r = reg();
    assert!(r
        .try_register(TryRegisterRequest {
            id: "a".to_string(),
            entry: Arc::from("alpha"),
        })
        .is_ok());
    assert_eq!(get(&r, "a").as_deref(), Some("alpha"));
}

/// @covers: Registry::try_register
#[test]
fn test_try_register_duplicate_id_returns_err_error() {
    let r = reg();
    register(&r, "a", "alpha");
    let result = r.try_register(TryRegisterRequest {
        id: "a".to_string(),
        entry: Arc::from("beta"),
    });
    assert_eq!(result, Err(RegistryError::DuplicateId("a".to_string())));
    assert_eq!(get(&r, "a").as_deref(), Some("alpha"));
}

/// @covers: Registry::try_register
#[test]
fn test_try_register_after_deregister_succeeds_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    r.deregister(DeregisterRequest {
        id: "a".to_string(),
    })
    .unwrap();
    assert!(r
        .try_register(TryRegisterRequest {
            id: "a".to_string(),
            entry: Arc::from("beta"),
        })
        .is_ok());
}

// ── deregister ────────────────────────────────────────────────────────────────
/// @covers: Registry::deregister
#[test]
fn test_deregister_existing_removes_and_returns_true_happy() {
    let r = reg();
    register(&r, "a", "alpha");
    assert!(
        r.deregister(DeregisterRequest {
            id: "a".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(get(&r, "a").is_none());
}

/// @covers: Registry::deregister
#[test]
fn test_deregister_missing_returns_false_error() {
    let r = reg();
    assert!(
        !r.deregister(DeregisterRequest {
            id: "absent".to_string()
        })
        .unwrap()
        .was_present
    );
}

/// @covers: Registry::deregister
#[test]
fn test_deregister_twice_second_returns_false_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    assert!(
        r.deregister(DeregisterRequest {
            id: "a".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(
        !r.deregister(DeregisterRequest {
            id: "a".to_string()
        })
        .unwrap()
        .was_present
    );
}

// ── get ───────────────────────────────────────────────────────────────────────
/// @covers: Registry::get
#[test]
fn test_get_registered_id_returns_entry_happy() {
    let r = reg();
    register(&r, "a", "alpha");
    assert_eq!(get(&r, "a").as_deref(), Some("alpha"));
}

/// @covers: Registry::get
#[test]
fn test_get_unregistered_id_returns_none_error() {
    let r = reg();
    assert!(get(&r, "absent").is_none());
}

/// @covers: Registry::get
#[test]
fn test_get_after_replace_returns_latest_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    register(&r, "a", "beta");
    assert_eq!(get(&r, "a").as_deref(), Some("beta"));
}

// ── list_ids ──────────────────────────────────────────────────────────────────
/// @covers: Registry::list_ids
#[test]
fn test_list_ids_returns_all_registered_happy() {
    let r = reg();
    register(&r, "a", "alpha");
    register(&r, "b", "beta");
    let mut ids = r.list_ids(ListIdsRequest).unwrap().ids;
    ids.sort();
    assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
}

/// @covers: Registry::list_ids
#[test]
fn test_list_ids_empty_registry_returns_empty_error() {
    let r = reg();
    assert!(r.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

/// @covers: Registry::list_ids
#[test]
fn test_list_ids_excludes_deregistered_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    register(&r, "b", "beta");
    r.deregister(DeregisterRequest {
        id: "a".to_string(),
    })
    .unwrap();
    assert_eq!(
        r.list_ids(ListIdsRequest).unwrap().ids,
        vec!["b".to_string()]
    );
}

// ── len ───────────────────────────────────────────────────────────────────────
/// @covers: Registry::len
#[test]
fn test_len_counts_registered_entries_happy() {
    let r = reg();
    register(&r, "a", "alpha");
    register(&r, "b", "beta");
    assert_eq!(r.len(LenRequest).unwrap().count, 2);
}

/// @covers: Registry::len
#[test]
fn test_len_empty_registry_is_zero_error() {
    let r = reg();
    assert_eq!(r.len(LenRequest).unwrap().count, 0);
}

/// @covers: Registry::len
#[test]
fn test_len_unchanged_after_replace_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    register(&r, "a", "beta");
    assert_eq!(r.len(LenRequest).unwrap().count, 1);
}

// ── is_empty ──────────────────────────────────────────────────────────────────
/// @covers: Registry::is_empty
#[test]
fn test_is_empty_new_registry_is_true_happy() {
    let r = reg();
    assert!(r.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: Registry::is_empty
#[test]
fn test_is_empty_after_register_is_false_error() {
    let r = reg();
    register(&r, "a", "alpha");
    assert!(!r.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: Registry::is_empty
#[test]
fn test_is_empty_true_again_after_clearing_edge() {
    let r = reg();
    register(&r, "a", "alpha");
    r.deregister(DeregisterRequest {
        id: "a".to_string(),
    })
    .unwrap();
    assert!(r.is_empty(EmptinessRequest).unwrap().empty);
}
