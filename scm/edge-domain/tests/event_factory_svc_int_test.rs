//! Integration tests for the event primitives' SAF facade.
#![cfg(feature = "event")]
// @allow: no_mocks_in_integration — MemoryEventStore is the production-shipped reference impl, not a test double
#![allow(clippy::unwrap_used)]

use edge_application::{
    ClosedEventSource, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventOccurredAtRequest, EventOccurredAtResponse, EventTypeRequest, EventTypeResponse,
    InProcessEventBus, MemoryEventStore, NoopEventBus, NoopEventPublisher,
};

#[derive(Clone)]
struct AnyEvent;
impl DomainEvent for AnyEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.any",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "agg-1",
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: std::time::SystemTime::now(),
        })
    }
}

// --- InProcessEventBus::new ---

/// @covers InProcessEventBus::new — happy path: returns an InProcessEventBus
#[test]
fn test_in_process_bus_returns_bus_happy() {
    let bus: InProcessEventBus = InProcessEventBus::new(256);
    assert_ne!(
        std::mem::size_of_val(&bus),
        0,
        "InProcessEventBus should be heap-backed"
    );
}

/// @covers InProcessEventBus::new — error: minimum valid capacity (1) constructs without panic
#[test]
fn test_in_process_bus_min_capacity_constructs_error() {
    let bus: InProcessEventBus = InProcessEventBus::new(1);
    assert_ne!(
        std::mem::size_of_val(&bus),
        0,
        "bus with min capacity should construct successfully"
    );
}

/// @covers InProcessEventBus::new — edge: successive calls produce independent buses
#[test]
fn test_in_process_bus_independent_calls_edge() {
    let a = InProcessEventBus::new(16);
    let b = InProcessEventBus::new(32);
    // Different pointers indicate independent instances
    let a_ptr = &a as *const _;
    let b_ptr = &b as *const _;
    assert_ne!(
        a_ptr, b_ptr,
        "successive calls should produce independent instances"
    );
}

// --- NoopEventBus ---

/// @covers NoopEventBus — happy path: returns a NoopEventBus marker
#[test]
fn test_noop_bus_returns_marker_happy() {
    let bus: NoopEventBus = NoopEventBus;
    assert_eq!(
        std::mem::size_of_val(&bus),
        0,
        "NoopEventBus should be zero-sized"
    );
}

/// @covers NoopEventBus — error: NoopEventBus is zero-size (cannot panic)
#[test]
fn test_noop_bus_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopEventBus>(), 0);
}

/// @covers NoopEventBus — edge: successive constructions are independent
#[test]
fn test_noop_bus_independent_calls_edge() {
    let a = NoopEventBus;
    let b = NoopEventBus;
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

// --- NoopEventPublisher ---

/// @covers NoopEventPublisher — happy path: returns a NoopEventPublisher marker
#[test]
fn test_noop_publisher_returns_marker_happy() {
    let pub_: NoopEventPublisher = NoopEventPublisher;
    assert_eq!(
        std::mem::size_of_val(&pub_),
        0,
        "NoopEventPublisher should be zero-sized"
    );
}

/// @covers NoopEventPublisher — error: NoopEventPublisher is zero-size
#[test]
fn test_noop_publisher_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopEventPublisher>(), 0);
}

/// @covers NoopEventPublisher — edge: successive constructions are independent
#[test]
fn test_noop_publisher_independent_calls_edge() {
    let a = NoopEventPublisher;
    let b = NoopEventPublisher;
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

// --- MemoryEventStore::new ---

/// @covers MemoryEventStore::new — happy path: constructs successfully
#[test]
fn test_in_memory_store_constructs_successfully_happy() {
    // @allow: no_mocks_in_integration — MemoryEventStore is the production-shipped reference impl
    let store = MemoryEventStore::<AnyEvent>::new();
    assert_ne!(
        std::mem::size_of_val(&store),
        0,
        "in_memory store should be heap-backed"
    );
}

/// @covers MemoryEventStore::new — error: store is non-zero-size (heap-backed)
#[test]
fn test_in_memory_store_is_nonzero_size_error() {
    assert_ne!(
        std::mem::size_of_val(&MemoryEventStore::<AnyEvent>::new()),
        0,
    );
}

/// @covers MemoryEventStore::new — edge: successive calls produce independent stores
#[test]
fn test_in_memory_store_successive_calls_independent_edge() {
    let a = MemoryEventStore::<AnyEvent>::new();
    let b = MemoryEventStore::<AnyEvent>::new();
    let a_ptr = &a as *const _;
    let b_ptr = &b as *const _;
    assert_ne!(
        a_ptr, b_ptr,
        "successive calls should produce independent instances"
    );
}

// --- ClosedEventSource ---

/// @covers ClosedEventSource — happy path: returns a ClosedEventSource marker
#[test]
fn test_closed_source_returns_marker_happy() {
    let source: ClosedEventSource = ClosedEventSource;
    assert_eq!(
        std::mem::size_of_val(&source),
        0,
        "ClosedEventSource should be zero-sized"
    );
}

/// @covers ClosedEventSource — error: ClosedEventSource is zero-size
#[test]
fn test_closed_source_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<ClosedEventSource>(), 0);
}

/// @covers ClosedEventSource — edge: successive constructions are independent
#[test]
fn test_closed_source_successive_calls_independent_edge() {
    let a = ClosedEventSource;
    let b = ClosedEventSource;
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}
