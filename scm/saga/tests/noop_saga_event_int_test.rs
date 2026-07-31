//! Integration tests for [`NoopSagaEvent`].
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{EventAggregateIdRequest, EventTypeRequest};
use edge_application_saga::{DomainEvent, NoopSagaEvent};

/// @covers: aggregate_id
#[test]
fn test_aggregate_id_noop_saga_event_returns_empty_happy() {
    let evt = NoopSagaEvent;
    assert_eq!(
        evt.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}

/// @covers: event_type
#[test]
fn test_event_type_noop_saga_event_returns_default_error() {
    // Verifies the DomainEvent default impl is inherited, not accidentally overridden
    let evt = NoopSagaEvent;
    assert_eq!(
        evt.event_type(EventTypeRequest).unwrap().event_type,
        "event"
    );
}

/// @covers: aggregate_id
#[test]
fn test_clone_noop_saga_event_preserves_empty_aggregate_id_edge() {
    let original = NoopSagaEvent;
    let cloned = original.clone();
    assert_eq!(
        cloned
            .aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}
