//! SAF facade tests — `EventStore` trait via `MemoryEventStore`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventStore, EventStoreAppendRequest, EventStoreError,
    EventStoreLoadFromRequest, EventStoreLoadRequest, EventTypeRequest, ExpectedVersion,
    MemoryEventStore,
};

#[derive(Clone)]
struct OrderEvt(String);
impl DomainEvent for OrderEvt {
    fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventTypeResponse { event_type: "order.evt" })
    }
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<edge_application_event::EventAggregateIdResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventAggregateIdResponse { aggregate_id: &self.0 })
    }
}

/// @covers: MemoryEventStore::append — first write with Any succeeds
#[test]
fn test_append_any_first_write_returns_sequence_happy() {
    let store = MemoryEventStore::<OrderEvt>::new();
    let resp = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a1",
        events: vec![OrderEvt("a1".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    assert_eq!(resp.sequence, 1);
}

/// @covers: MemoryEventStore::load — returns empty for unknown aggregate
#[test]
fn test_load_unknown_aggregate_returns_empty_error() {
    let store = MemoryEventStore::<OrderEvt>::new();
    let events = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "unknown" }),
    )
    .expect("load")
    .events;
    assert!(events.is_empty());
}

/// @covers: MemoryEventStore::append+load — round trip
#[test]
fn test_append_then_load_returns_appended_events_happy() {
    let store = MemoryEventStore::<OrderEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a2",
        events: vec![OrderEvt("a2".into()), OrderEvt("a2".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "a2" }),
    )
    .expect("load")
    .events;
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 1);
    assert_eq!(events[1].sequence, 2);
}

/// @covers: MemoryEventStore::load_from — returns only events from sequence
#[test]
fn test_load_from_sequence_2_skips_first_event_edge() {
    let store = MemoryEventStore::<OrderEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a3",
        events: vec![OrderEvt("a3".into()), OrderEvt("a3".into()), OrderEvt("a3".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "a3", from_sequence: 2 }),
    )
    .expect("load_from")
    .events;
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 2);
}

/// @covers: EventStoreError::Conflict — NoStream after stream exists
#[test]
fn test_append_no_stream_conflict_returns_err_error() {
    let store = MemoryEventStore::<OrderEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a4",
        events: vec![OrderEvt("a4".into())],
        expected: ExpectedVersion::NoStream,
    }))
    .expect("first");
    let err = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a4",
        events: vec![OrderEvt("a4".into())],
        expected: ExpectedVersion::NoStream,
    }))
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: MemoryEventStore::append — Exact version mismatch returns conflict
#[test]
fn test_append_exact_wrong_version_returns_conflict_edge() {
    let store = MemoryEventStore::<OrderEvt>::new();
    let err = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a5",
        events: vec![OrderEvt("a5".into())],
        expected: ExpectedVersion::Exact(99),
    }))
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: MemoryEventStore::load — returns appended events for known aggregate
#[test]
fn test_load_known_aggregate_returns_events_happy() {
    let store = MemoryEventStore::<OrderEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a6",
        events: vec![OrderEvt("a6".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "a6" }),
    )
    .expect("load")
    .events;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].aggregate_id, "a6");
}

/// @covers: MemoryEventStore::load_from — returns events from sequence 1
#[test]
fn test_load_from_sequence_1_returns_all_events_happy() {
    let store = MemoryEventStore::<OrderEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "a7",
        events: vec![OrderEvt("a7".into()), OrderEvt("a7".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "a7", from_sequence: 1 }),
    )
    .expect("load_from")
    .events;
    assert_eq!(events.len(), 2);
}

/// @covers: MemoryEventStore::load_from — unknown aggregate returns empty
#[test]
fn test_load_from_unknown_aggregate_returns_empty_error() {
    let store = MemoryEventStore::<OrderEvt>::new();
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "missing", from_sequence: 1 }),
    )
    .expect("load_from")
    .events;
    assert!(events.is_empty());
}
