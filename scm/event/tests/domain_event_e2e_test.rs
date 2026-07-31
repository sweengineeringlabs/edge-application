//! SAF facade tests — `DomainEvent` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventOccurredAtRequest, EventTypeRequest,
};
use std::time::SystemTime;

struct Order {
    id: String,
}

impl DomainEvent for Order {
    fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventTypeResponse { event_type: "order.created" })
    }
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<edge_application_event::EventAggregateIdResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventAggregateIdResponse { aggregate_id: &self.id })
    }
}

struct Bare;
impl DomainEvent for Bare {}

/// @covers: DomainEvent::event_type — custom type returned
#[test]
fn test_event_type_custom_returns_value_happy() {
    assert_eq!(
        Order { id: "x".into() }.event_type(EventTypeRequest).unwrap().event_type,
        "order.created"
    );
}

/// @covers: DomainEvent::event_type — default impl returns "event"
#[test]
fn test_event_type_default_returns_event_error() {
    assert_eq!(Bare.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: DomainEvent::aggregate_id — custom id returned
#[test]
fn test_aggregate_id_custom_value_returned_happy() {
    assert_eq!(
        Order { id: "agg-1".into() }.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id,
        "agg-1"
    );
}

/// @covers: DomainEvent::aggregate_id — default impl returns empty string
#[test]
fn test_aggregate_id_default_returns_empty_error() {
    assert_eq!(Bare.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id, "");
}

/// @covers: DomainEvent::event_type — custom type with unicode chars
#[test]
fn test_event_type_unicode_value_returned_edge() {
    struct UnicodeEvt;
    impl DomainEvent for UnicodeEvt {
        fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, edge_application_event::EventError> {
            Ok(edge_application_event::EventTypeResponse { event_type: "order.créé" })
        }
    }
    assert_eq!(UnicodeEvt.event_type(EventTypeRequest).unwrap().event_type, "order.créé");
}

/// @covers: DomainEvent::aggregate_id — id with special chars
#[test]
fn test_aggregate_id_special_chars_returned_edge() {
    struct SpecialEvt;
    impl DomainEvent for SpecialEvt {
        fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<edge_application_event::EventAggregateIdResponse<'_>, edge_application_event::EventError> {
            Ok(edge_application_event::EventAggregateIdResponse { aggregate_id: "agg/1:v2" })
        }
    }
    assert_eq!(SpecialEvt.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id, "agg/1:v2");
}

/// @covers: DomainEvent::occurred_at — default returns a recent time
#[test]
fn test_occurred_at_default_returns_recent_time_happy() {
    let before = SystemTime::now();
    let t = Bare.occurred_at(EventOccurredAtRequest).unwrap().occurred_at;
    let after = SystemTime::now();
    assert!(t >= before);
    assert!(t <= after);
}

/// @covers: DomainEvent::occurred_at — custom impl returns fixed time
#[test]
fn test_occurred_at_custom_impl_returns_fixed_time_error() {
    use std::time::{Duration, UNIX_EPOCH};
    struct TimedEvt;
    impl DomainEvent for TimedEvt {
        fn occurred_at(&self, _req: EventOccurredAtRequest) -> Result<edge_application_event::EventOccurredAtResponse, edge_application_event::EventError> {
            Ok(edge_application_event::EventOccurredAtResponse { occurred_at: UNIX_EPOCH + Duration::from_secs(1_000_000) })
        }
    }
    assert_eq!(
        TimedEvt.occurred_at(EventOccurredAtRequest).unwrap().occurred_at,
        UNIX_EPOCH + Duration::from_secs(1_000_000)
    );
}

/// @covers: DomainEvent::occurred_at — two consecutive calls return non-decreasing times
#[test]
fn test_occurred_at_default_two_calls_non_decreasing_edge() {
    let t1 = Bare.occurred_at(EventOccurredAtRequest).unwrap().occurred_at;
    let t2 = Bare.occurred_at(EventOccurredAtRequest).unwrap().occurred_at;
    assert!(t2 >= t1);
}
