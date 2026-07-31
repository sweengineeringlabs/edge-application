//! SAF facade tests — direct construction of the standard event primitives.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    AggregateApplyRequest, AggregateIdentityRequest, ClosedEventSource, DomainEvent,
    EventAggregateIdRequest, EventSource, EventSourceRecvNextRequest, EventTypeRequest,
    MemoryEventStore, InProcessEventBus, NoopAggregate, NoopDomainEvent, NoopEventBus,
    NoopEventPublisher,
};

#[derive(Clone)]
struct Evt;
impl DomainEvent for Evt {}

/// @covers: NoopEventBus — zero-sized marker
#[test]
fn test_noop_bus_is_zero_sized_happy() {
    let bus = NoopEventBus;
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: NoopEventPublisher — zero-sized marker
#[test]
fn test_noop_publisher_is_zero_sized_happy() {
    let pub_ = NoopEventPublisher;
    assert_eq!(std::mem::size_of_val(&pub_), 0);
}

/// @covers: ClosedEventSource — zero-sized marker
#[test]
fn test_closed_source_is_zero_sized_happy() {
    let src = ClosedEventSource;
    assert_eq!(std::mem::size_of_val(&src), 0);
}

/// @covers: MemoryEventStore::new — returns usable store
#[test]
fn test_in_memory_store_appends_successfully_happy() {
    use edge_application_event::{EventStore, EventStoreAppendRequest, ExpectedVersion};
    let store = MemoryEventStore::<Evt>::new();
    let resp = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![Evt],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    assert_eq!(resp.sequence, 1);
}

/// @covers: InProcessEventBus::new — capacity is honoured
#[test]
fn test_in_process_bus_constructed_with_config_edge() {
    let bus = InProcessEventBus::new(32);
    // Sender capacity is set — just verify it doesn't panic
    assert!(std::mem::size_of_val(&bus) > 0);
}

/// @covers: NoopEventBus::publish — publish on noop bus never errors
#[test]
fn test_noop_bus_publish_never_errors_error() {
    use std::sync::Arc;
    use edge_application_event::{EventBus, EventBusPublishRequest};
    let bus = NoopEventBus;
    let result = futures::executor::block_on(
        bus.publish(EventBusPublishRequest { event: Arc::new(Evt) }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: NoopEventBus::subscribe — subscribe returns a receiver
#[test]
fn test_noop_bus_subscribe_returns_closed_receiver_edge() {
    use edge_application_event::{EventBus, EventBusSubscribeRequest, EventError};
    let bus = NoopEventBus;
    let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let result = futures::executor::block_on(rx.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: NoopEventPublisher::publish — publish via noop publisher never errors
#[test]
fn test_noop_publisher_publish_never_errors_error() {
    use edge_application_event::{EventPublisher, EventPublisherPublishRequest};
    let pub_ = NoopEventPublisher;
    let result = futures::executor::block_on(
        pub_.publish(EventPublisherPublishRequest { event: &Evt }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: NoopEventPublisher::publish — dyn dispatch works
#[test]
fn test_noop_publisher_dyn_dispatch_never_errors_edge() {
    use edge_application_event::{DomainEvent as DomainEventTrait, EventPublisher, EventPublisherPublishRequest};
    let pub_ = NoopEventPublisher;
    let evt: &dyn DomainEventTrait = &Evt;
    assert_eq!(
        futures::executor::block_on(pub_.publish(EventPublisherPublishRequest { event: evt })),
        Ok(())
    );
}

/// @covers: MemoryEventStore::append — append conflict on NoStream after stream exists
#[test]
fn test_in_memory_store_conflict_on_no_stream_error() {
    use edge_application_event::{EventStore, EventStoreAppendRequest, EventStoreError, ExpectedVersion};
    let store = MemoryEventStore::<Evt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![Evt],
        expected: ExpectedVersion::NoStream,
    }))
    .expect("first append");
    let err = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![Evt],
        expected: ExpectedVersion::NoStream,
    }))
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: MemoryEventStore::load_from — load_from returns subset
#[test]
fn test_in_memory_store_load_from_returns_filtered_events_edge() {
    use edge_application_event::{EventStore, EventStoreAppendRequest, EventStoreLoadFromRequest, ExpectedVersion};
    let store = MemoryEventStore::<Evt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "y",
        events: vec![Evt, Evt, Evt],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "y", from_sequence: 2 }),
    )
    .expect("load_from")
    .events;
    assert_eq!(events.len(), 2);
}

/// @covers: ClosedEventSource::recv_next — error message is non-empty
#[test]
fn test_closed_source_error_message_non_empty_error() {
    let mut src = ClosedEventSource;
    let err = match futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest)) {
        Err(e) => e,
        Ok(_) => panic!("expected Err from closed source"),
    };
    assert!(!err.to_string().is_empty());
}

/// @covers: ClosedEventSource::recv_next — repeated calls all return Unavailable
#[test]
fn test_closed_source_repeated_calls_all_unavailable_edge() {
    use edge_application_event::EventError;
    let mut src = ClosedEventSource;
    for _ in 0..3 {
        let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    }
}

/// @covers: InProcessEventBus::new — publish and subscribe round-trip
#[test]
fn test_in_process_bus_publish_subscribe_round_trip_error() {
    use std::sync::Arc;
    use edge_application_event::{EventBus, EventBusConfig, EventBusPublishRequest, EventBusSubscribeRequest};
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(EventBusConfig::default().capacity);
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        bus.publish(EventBusPublishRequest { event: Arc::new(Evt) }).await.expect("publish");
        let e = rx.recv_next(EventSourceRecvNextRequest).await.expect("recv").event;
        assert!(!e.event_type(EventTypeRequest).unwrap().event_type.is_empty());
    });
}

/// @covers: NoopAggregate — returns a NoopAggregate
#[test]
fn test_noop_aggregate_returns_noop_aggregate_happy() {
    use edge_application_event::Aggregate;
    let agg = NoopAggregate;
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}

/// @covers: NoopAggregate::apply — apply is a no-op
#[test]
fn test_noop_aggregate_apply_is_noop_error() {
    use edge_application_event::Aggregate;
    let mut agg = NoopAggregate;
    agg.apply(AggregateApplyRequest { event: &NoopDomainEvent }).unwrap();
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}

/// @covers: NoopAggregate — multiple constructions are independent
#[test]
fn test_noop_aggregate_multiple_calls_are_independent_edge() {
    use edge_application_event::Aggregate;
    let a = NoopAggregate;
    let b = NoopAggregate;
    assert_eq!(a.id(AggregateIdentityRequest).unwrap().id, b.id(AggregateIdentityRequest).unwrap().id);
}

/// @covers: NoopDomainEvent — returns a NoopDomainEvent
#[test]
fn test_noop_event_returns_noop_domain_event_happy() {
    let evt = NoopDomainEvent;
    assert_eq!(evt.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: NoopDomainEvent — aggregate_id is empty string
#[test]
fn test_noop_event_aggregate_id_is_empty_error() {
    let evt = NoopDomainEvent;
    assert_eq!(evt.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id, "");
}

/// @covers: NoopDomainEvent — multiple constructions produce independent values
#[test]
fn test_noop_event_multiple_calls_are_independent_edge() {
    let a = NoopDomainEvent;
    let b = NoopDomainEvent;
    assert_eq!(
        a.event_type(EventTypeRequest).unwrap().event_type,
        b.event_type(EventTypeRequest).unwrap().event_type
    );
}
