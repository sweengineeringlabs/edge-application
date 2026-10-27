//! Integration tests for `OutboundRegistry` via `MemoryOutboundRegistry`.
// @allow: no_mocks_in_integration — MemoryOutboundRegistry is the production-shipped reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    MemoryOutboundRegistry, OutboundDeregisterRequest, OutboundGetRequest, OutboundIsEmptyRequest,
    OutboundLenRequest, OutboundNamesRequest, OutboundRegisterRequest, OutboundRegistry,
};

/// @covers: OutboundRegistry::is_empty — new registry starts empty
#[test]
fn test_outbound_registry_new_creates_empty_registry_happy() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::new();
    assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
    assert_eq!(reg.len(OutboundLenRequest).unwrap().count, 0);
}

/// @covers: OutboundRegistry::register — stores a handle retrievable via get
#[test]
fn test_outbound_registry_register_stores_handle_happy() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "svc".into(),
        handle: "url".to_string(),
    })
    .unwrap();
    let handle = reg
        .get(OutboundGetRequest { name: "svc".into() })
        .unwrap()
        .handle;
    assert_eq!(handle.as_deref(), Some("url"));
}

/// @covers: OutboundRegistry::deregister — removes a registered handle
#[test]
fn test_outbound_registry_deregister_removes_handle_happy() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "svc".into(),
        handle: "url".to_string(),
    })
    .unwrap();
    let removed = reg
        .deregister(OutboundDeregisterRequest { name: "svc".into() })
        .unwrap()
        .removed;
    assert!(removed);
    assert!(reg
        .get(OutboundGetRequest { name: "svc".into() })
        .unwrap()
        .handle
        .is_none());
}

/// @covers: OutboundRegistry::deregister — missing name reports not removed
#[test]
fn test_outbound_registry_deregister_missing_name_returns_false_error() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::new();
    let removed = reg
        .deregister(OutboundDeregisterRequest {
            name: "missing".into(),
        })
        .unwrap()
        .removed;
    assert!(!removed);
}

/// @covers: OutboundRegistry::names — returns every registered name
#[test]
fn test_outbound_registry_names_returns_all_registered_edge() {
    let reg: MemoryOutboundRegistry<u32> = MemoryOutboundRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: 1,
    })
    .unwrap();
    reg.register(OutboundRegisterRequest {
        name: "b".into(),
        handle: 2,
    })
    .unwrap();
    let mut names = reg.names(OutboundNamesRequest).unwrap().names;
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
}

/// @covers: OutboundRegistry::len — reflects registered count
#[test]
fn test_outbound_registry_len_reflects_count_happy() {
    let reg: MemoryOutboundRegistry<u32> = MemoryOutboundRegistry::new();
    assert_eq!(reg.len(OutboundLenRequest).unwrap().count, 0);
    reg.register(OutboundRegisterRequest {
        name: "x".into(),
        handle: 1,
    })
    .unwrap();
    assert_eq!(reg.len(OutboundLenRequest).unwrap().count, 1);
}

/// @covers: OutboundRegistry::get — usable via dyn dispatch
#[test]
fn test_outbound_registry_via_dyn_dispatch_returns_handle_edge() {
    let reg: MemoryOutboundRegistry<String> = MemoryOutboundRegistry::new();
    let dyn_reg: &dyn OutboundRegistry<Handle = String> = &reg;
    dyn_reg
        .register(OutboundRegisterRequest {
            name: "dyn".into(),
            handle: "value".to_string(),
        })
        .unwrap();
    let handle = dyn_reg
        .get(OutboundGetRequest { name: "dyn".into() })
        .unwrap()
        .handle;
    assert_eq!(handle.as_deref(), Some("value"));
}
