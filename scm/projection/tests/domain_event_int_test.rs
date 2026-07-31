//! Integration coverage for the `edge-domain-event` dependency used in projection impls.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{DomainEvent, EventAggregateIdRequest, EventOccurredAtRequest, EventTypeRequest};

#[derive(Clone)]
struct DomainEventTestEvt;

impl DomainEvent for DomainEventTestEvt {}

#[test]
fn test_domain_event_default_event_type_returns_event_happy() {
    let e = DomainEventTestEvt;
    assert_eq!(e.event_type(EventTypeRequest).unwrap().event_type, "event");
}

#[test]
fn test_domain_event_default_aggregate_id_returns_empty_error() {
    let e = DomainEventTestEvt;
    assert_eq!(e.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id, "");
}

#[test]
fn test_domain_event_occurred_at_returns_system_time_edge() {
    let e = DomainEventTestEvt;
    let t = e.occurred_at(EventOccurredAtRequest).unwrap().occurred_at;
    let elapsed = t.elapsed();
    assert!(elapsed.is_ok(), "should be able to calculate elapsed time from occurred_at");
    let _duration = elapsed.unwrap();
}
