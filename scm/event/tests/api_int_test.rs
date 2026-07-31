//! Layer-level coverage for `api/event/dto/*.rs` request/response types.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::SystemTime;

use edge_application_event::{
    Aggregate, AggregateApplyRequest, AggregateApplyResponse, AggregateIdentityRequest,
    AggregateIdentityResponse, ClosedEventSource, DomainEvent, EventAggregateIdRequest,
    EventAggregateIdResponse, EventBus, EventBusPublishRequest, EventBusSubscribeRequest,
    EventBusSubscribeResponse, EventError, EventOccurredAtRequest, EventOccurredAtResponse,
    EventPublisher, EventPublisherPublishRequest, EventSource, EventSourceRecvNextRequest,
    EventSourceRecvNextResponse, EventStore, EventStoreAppendRequest, EventStoreAppendResponse,
    EventStoreLoadFromRequest, EventStoreLoadFromResponse, EventStoreLoadRequest,
    EventStoreLoadResponse, EventTypeRequest, EventTypeResponse, ExpectedVersion,
    MemoryEventStore, InProcessEventBus, NoopAggregate, NoopDomainEvent,
};

/// @covers: AggregateApplyRequest
#[test]
fn test_aggregate_apply_request_wraps_event_reference_happy() {
    let evt = NoopDomainEvent;
    let req = AggregateApplyRequest { event: &evt };
    assert_eq!(req.event.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: AggregateApplyRequest
#[test]
fn test_aggregate_apply_request_used_by_noop_aggregate_apply_edge() {
    let mut agg = NoopAggregate;
    let result = agg.apply(AggregateApplyRequest { event: &NoopDomainEvent });
    assert_eq!(result, Ok(AggregateApplyResponse));
}

/// @covers: AggregateApplyResponse
#[test]
fn test_aggregate_apply_response_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<AggregateApplyResponse>(), 0);
}

/// @covers: AggregateApplyResponse
#[test]
fn test_aggregate_apply_response_returned_by_apply_error() {
    let mut agg = NoopAggregate;
    let resp: AggregateApplyResponse =
        agg.apply(AggregateApplyRequest { event: &NoopDomainEvent }).unwrap();
    assert_eq!(std::mem::size_of_val(&resp), 0);
}

/// @covers: AggregateIdentityRequest
#[test]
fn test_aggregate_identity_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<AggregateIdentityRequest>(), 0);
}

/// @covers: AggregateIdentityRequest
#[test]
fn test_aggregate_identity_request_used_by_noop_aggregate_id_edge() {
    let agg = NoopAggregate;
    let resp = agg.id(AggregateIdentityRequest).unwrap();
    assert_eq!(resp.id, "");
}

/// @covers: AggregateIdentityResponse
#[test]
fn test_aggregate_identity_response_exposes_id_field_happy() {
    let resp = AggregateIdentityResponse { id: "agg-42" };
    assert_eq!(resp.id, "agg-42");
}

/// @covers: AggregateIdentityResponse
#[test]
fn test_aggregate_identity_response_empty_id_edge() {
    let resp = AggregateIdentityResponse { id: "" };
    assert!(resp.id.is_empty());
}

/// @covers: EventAggregateIdRequest
#[test]
fn test_event_aggregate_id_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EventAggregateIdRequest>(), 0);
}

/// @covers: EventAggregateIdRequest
#[test]
fn test_event_aggregate_id_request_used_by_noop_event_edge() {
    let evt = NoopDomainEvent;
    let resp = evt.aggregate_id(EventAggregateIdRequest).unwrap();
    assert_eq!(resp.aggregate_id, "");
}

/// @covers: EventAggregateIdResponse
#[test]
fn test_event_aggregate_id_response_exposes_field_happy() {
    let resp = EventAggregateIdResponse { aggregate_id: "order-1" };
    assert_eq!(resp.aggregate_id, "order-1");
}

/// @covers: EventAggregateIdResponse
#[test]
fn test_event_aggregate_id_response_special_chars_edge() {
    let resp = EventAggregateIdResponse { aggregate_id: "agg/1:v2" };
    assert_eq!(resp.aggregate_id, "agg/1:v2");
}

/// @covers: EventBusPublishRequest
#[test]
fn test_event_bus_publish_request_wraps_arc_dyn_domain_event_happy() {
    let req = EventBusPublishRequest { event: Arc::new(NoopDomainEvent) };
    assert_eq!(req.event.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: EventBusPublishRequest
#[test]
fn test_event_bus_publish_request_accepted_by_noop_bus_edge() {
    use edge_application_event::NoopEventBus;
    let result = futures::executor::block_on(
        NoopEventBus.publish(EventBusPublishRequest { event: Arc::new(NoopDomainEvent) }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: EventBusSubscribeRequest
#[test]
fn test_event_bus_subscribe_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EventBusSubscribeRequest>(), 0);
}

/// @covers: EventBusSubscribeRequest
#[test]
fn test_event_bus_subscribe_request_accepted_by_noop_bus_edge() {
    use edge_application_event::NoopEventBus;
    let mut resp = NoopEventBus
        .subscribe(EventBusSubscribeRequest)
        .expect("subscribe");
    let recv_result =
        futures::executor::block_on(resp.receiver.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(recv_result, Err(EventError::Unavailable(_))));
}

/// @covers: EventBusSubscribeResponse
#[test]
fn test_event_bus_subscribe_response_exposes_receiver_happy() {
    use edge_application_event::NoopEventBus;
    let resp: EventBusSubscribeResponse = NoopEventBus.subscribe(EventBusSubscribeRequest).unwrap();
    let mut rx = resp.receiver;
    let result = futures::executor::block_on(rx.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: EventBusSubscribeResponse
#[test]
fn test_event_bus_subscribe_response_from_in_process_bus_edge() {
    let bus = InProcessEventBus::new(4);
    let resp: EventBusSubscribeResponse = bus.subscribe(EventBusSubscribeRequest).unwrap();
    // Just verify the receiver is constructed and usable (non-panicking drop).
    drop(resp.receiver);
}

/// @covers: EventOccurredAtRequest
#[test]
fn test_event_occurred_at_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EventOccurredAtRequest>(), 0);
}

/// @covers: EventOccurredAtRequest
#[test]
fn test_event_occurred_at_request_used_by_noop_event_edge() {
    let evt = NoopDomainEvent;
    let before = SystemTime::now();
    let resp = evt.occurred_at(EventOccurredAtRequest).unwrap();
    let after = SystemTime::now();
    assert!(resp.occurred_at >= before && resp.occurred_at <= after);
}

/// @covers: EventOccurredAtResponse
#[test]
fn test_event_occurred_at_response_exposes_field_happy() {
    let now = SystemTime::now();
    let resp = EventOccurredAtResponse { occurred_at: now };
    assert_eq!(resp.occurred_at, now);
}

/// @covers: EventOccurredAtResponse
#[test]
fn test_event_occurred_at_response_epoch_boundary_edge() {
    let resp = EventOccurredAtResponse { occurred_at: SystemTime::UNIX_EPOCH };
    assert_eq!(resp.occurred_at, SystemTime::UNIX_EPOCH);
}

/// @covers: EventPublisherPublishRequest
#[test]
fn test_event_publisher_publish_request_wraps_dyn_domain_event_happy() {
    let evt = NoopDomainEvent;
    let req = EventPublisherPublishRequest { event: &evt };
    assert_eq!(req.event.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: EventPublisherPublishRequest
#[test]
fn test_event_publisher_publish_request_accepted_by_noop_publisher_edge() {
    use edge_application_event::NoopEventPublisher;
    let evt = NoopDomainEvent;
    let result = futures::executor::block_on(
        NoopEventPublisher.publish(EventPublisherPublishRequest { event: &evt }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: EventSourceRecvNextRequest
#[test]
fn test_event_source_recv_next_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EventSourceRecvNextRequest>(), 0);
}

/// @covers: EventSourceRecvNextRequest
#[test]
fn test_event_source_recv_next_request_used_by_closed_source_edge() {
    let mut src = ClosedEventSource;
    let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: EventStoreAppendRequest
#[test]
fn test_event_store_append_request_constructed_with_fields_happy() {
    let req = EventStoreAppendRequest {
        aggregate_id: "agg-1",
        events: vec![NoopDomainEvent],
        expected: ExpectedVersion::Any,
    };
    assert_eq!(req.aggregate_id, "agg-1");
    assert_eq!(req.events.len(), 1);
    assert_eq!(req.expected, ExpectedVersion::Any);
}

/// @covers: EventStoreAppendRequest
#[test]
fn test_event_store_append_request_used_by_in_memory_store_edge() {
    let store = MemoryEventStore::<NoopDomainEvent>::new();
    let resp = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "agg-api-test",
        events: vec![NoopDomainEvent],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    assert_eq!(resp.sequence, 1);
}

/// @covers: EventStoreAppendResponse
#[test]
fn test_event_store_append_response_exposes_sequence_field_happy() {
    let resp = EventStoreAppendResponse { sequence: 7 };
    assert_eq!(resp.sequence, 7);
}

/// @covers: EventStoreAppendResponse
#[test]
fn test_event_store_append_response_zero_boundary_edge() {
    let resp = EventStoreAppendResponse { sequence: 0 };
    assert_eq!(resp.sequence, 0);
}

/// @covers: EventStoreLoadFromRequest
#[test]
fn test_event_store_load_from_request_constructed_with_fields_happy() {
    let req = EventStoreLoadFromRequest { aggregate_id: "agg-1", from_sequence: 3 };
    assert_eq!(req.aggregate_id, "agg-1");
    assert_eq!(req.from_sequence, 3);
}

/// @covers: EventStoreLoadFromRequest
#[test]
fn test_event_store_load_from_request_used_by_in_memory_store_edge() {
    let store = MemoryEventStore::<NoopDomainEvent>::new();
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "missing", from_sequence: 1 }),
    )
    .expect("load_from")
    .events;
    assert!(events.is_empty());
}

/// @covers: EventStoreLoadRequest
#[test]
fn test_event_store_load_request_constructed_with_field_happy() {
    let req = EventStoreLoadRequest { aggregate_id: "agg-1" };
    assert_eq!(req.aggregate_id, "agg-1");
}

/// @covers: EventStoreLoadRequest
#[test]
fn test_event_store_load_request_used_by_in_memory_store_edge() {
    let store = MemoryEventStore::<NoopDomainEvent>::new();
    let events =
        futures::executor::block_on(store.load(EventStoreLoadRequest { aggregate_id: "missing" }))
            .expect("load")
            .events;
    assert!(events.is_empty());
}

/// @covers: EventTypeRequest
#[test]
fn test_event_type_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EventTypeRequest>(), 0);
}

/// @covers: EventTypeRequest
#[test]
fn test_event_type_request_used_by_noop_event_edge() {
    let evt = NoopDomainEvent;
    let resp = evt.event_type(EventTypeRequest).unwrap();
    assert_eq!(resp.event_type, "event");
}

/// @covers: EventTypeResponse
#[test]
fn test_event_type_response_exposes_field_happy() {
    let resp = EventTypeResponse { event_type: "order.created" };
    assert_eq!(resp.event_type, "order.created");
}

/// @covers: EventTypeResponse
#[test]
fn test_event_type_response_empty_string_edge() {
    let resp = EventTypeResponse { event_type: "" };
    assert!(resp.event_type.is_empty());
}

/// @covers: EventSourceRecvNextResponse
#[test]
fn test_event_source_recv_next_response_holds_event_happy() {
    let resp = EventSourceRecvNextResponse {
        event: Arc::new(NoopDomainEvent),
    };
    assert_eq!(
        resp.event.event_type(EventTypeRequest).unwrap().event_type,
        "event"
    );
}

/// @covers: EventSourceRecvNextResponse
#[test]
fn test_event_source_recv_next_response_used_by_closed_source_error() {
    use edge_application_event::EventSourceRecvNextRequest as Req;
    let mut src = ClosedEventSource;
    let result = futures::executor::block_on(src.recv_next(Req));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: EventStoreLoadResponse
#[test]
fn test_event_store_load_response_holds_events_happy() {
    let resp: EventStoreLoadResponse<NoopDomainEvent> = EventStoreLoadResponse { events: vec![] };
    assert!(resp.events.is_empty());
}

/// @covers: EventStoreLoadResponse
#[test]
fn test_event_store_load_response_used_by_in_memory_store_edge() {
    let store = MemoryEventStore::<NoopDomainEvent>::new();
    let resp = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "none" }),
    )
    .expect("load");
    assert_eq!(resp.events.len(), 0);
}

/// @covers: EventStoreLoadFromResponse
#[test]
fn test_event_store_load_from_response_holds_events_happy() {
    let resp: EventStoreLoadFromResponse<NoopDomainEvent> =
        EventStoreLoadFromResponse { events: vec![] };
    assert!(resp.events.is_empty());
}

/// @covers: EventStoreLoadFromResponse
#[test]
fn test_event_store_load_from_response_used_by_in_memory_store_edge() {
    let store = MemoryEventStore::<NoopDomainEvent>::new();
    let resp = futures::executor::block_on(store.load_from(EventStoreLoadFromRequest {
        aggregate_id: "none",
        from_sequence: 0,
    }))
    .expect("load_from");
    assert_eq!(resp.events.len(), 0);
}
