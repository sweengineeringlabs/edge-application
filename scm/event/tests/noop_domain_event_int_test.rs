//! Integration tests for `NoopDomainEvent`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventOccurredAtRequest, EventTypeRequest, NoopDomainEvent,
};
use std::time::SystemTime;

/// @covers: NoopDomainEvent — event_type returns the default "event" string
#[test]
fn test_event_type_noop_domain_event_returns_default_happy() {
    let evt = NoopDomainEvent;
    assert_eq!(evt.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: NoopDomainEvent — aggregate_id returns empty string
#[test]
fn test_aggregate_id_noop_domain_event_returns_empty_happy() {
    let evt = NoopDomainEvent;
    assert_eq!(evt.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id, "");
}

/// @covers: NoopDomainEvent — occurred_at returns a time within the test window
#[test]
fn test_occurred_at_noop_domain_event_within_test_window_edge() {
    let before = SystemTime::now();
    let evt = NoopDomainEvent;
    let at = evt.occurred_at(EventOccurredAtRequest).unwrap().occurred_at;
    let after = SystemTime::now();
    assert!(at >= before && at <= after);
}

/// @covers: NoopDomainEvent — Clone produces an independent copy
#[test]
fn test_clone_noop_domain_event_is_independent_edge() {
    let a = NoopDomainEvent;
    let b = a.clone();
    // Both are zero-sized — verify both are valid DomainEvent impls
    assert_eq!(
        a.event_type(EventTypeRequest).unwrap().event_type,
        b.event_type(EventTypeRequest).unwrap().event_type
    );
}

/// @covers: NoopDomainEvent — can be used as dyn DomainEvent
#[test]
fn test_noop_domain_event_satisfies_dyn_domain_event_happy() {
    let evt = NoopDomainEvent;
    let dyn_evt: &dyn DomainEvent = &evt;
    assert_eq!(dyn_evt.event_type(EventTypeRequest).unwrap().event_type, "event");
}
