//! Integration tests for `MemoryEventStore`.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventStore, EventStoreAppendRequest, EventStoreError,
    EventStoreLoadFromRequest, EventStoreLoadRequest, EventTypeRequest, ExpectedVersion,
    MemoryEventStore,
};

#[derive(Clone)]
struct ItemEvt(String);
impl DomainEvent for ItemEvt {
    fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventTypeResponse { event_type: "item.evt" })
    }
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<edge_application_event::EventAggregateIdResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventAggregateIdResponse { aggregate_id: &self.0 })
    }
}

/// @covers: MemoryEventStore::new — creates empty store (via factory)
#[test]
fn test_memory_event_store_new_creates_empty_store_happy() {
    let store = MemoryEventStore::<ItemEvt>::new();
    let events = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "any" }),
    )
    .expect("load")
    .events;
    assert!(events.is_empty());
}

/// @covers: MemoryEventStore::default — default store is usable
#[test]
fn test_memory_event_store_default_same_as_new_error() {
    let store = MemoryEventStore::<ItemEvt>::new();
    let result = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![ItemEvt("x".into())],
        expected: ExpectedVersion::Any,
    }));
    let resp = result.expect("append should succeed");
    assert_eq!(resp.sequence, 1);
}

/// @covers: MemoryEventStore — multiple aggregates are independent
#[test]
fn test_memory_event_store_multiple_aggregates_independent_edge() {
    let store = MemoryEventStore::<ItemEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "agg-a",
        events: vec![ItemEvt("agg-a".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append a");
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "agg-b",
        events: vec![ItemEvt("agg-b".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append b");
    let events_a = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "agg-a" }),
    )
    .expect("load a")
    .events;
    let events_b = futures::executor::block_on(
        store.load(EventStoreLoadRequest { aggregate_id: "agg-b" }),
    )
    .expect("load b")
    .events;
    assert_eq!(events_a.len(), 1);
    assert_eq!(events_b.len(), 1);
    assert_eq!(events_a[0].aggregate_id, "agg-a");
    assert_eq!(events_b[0].aggregate_id, "agg-b");
}

/// @covers: MemoryEventStore::append — Conflict error has correct fields
#[test]
fn test_memory_event_store_conflict_error_fields_correct_happy() {
    let store = MemoryEventStore::<ItemEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![ItemEvt("x".into())],
        expected: ExpectedVersion::NoStream,
    }))
    .expect("first");
    let err = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![ItemEvt("x".into())],
        expected: ExpectedVersion::NoStream,
    }))
    .unwrap_err();
    if let EventStoreError::Conflict { aggregate_id, expected, actual } = err {
        assert_eq!(aggregate_id, "x");
        assert_eq!(expected, 0);
        assert_eq!(actual, 1);
    } else {
        panic!("expected Conflict");
    }
}

/// @covers: MemoryEventStore::load_from — returns empty for future sequence
#[test]
fn test_memory_event_store_load_from_future_seq_returns_empty_error() {
    let store = MemoryEventStore::<ItemEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![ItemEvt("x".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "x", from_sequence: 999 }),
    )
    .expect("load_from")
    .events;
    assert!(events.is_empty());
}

/// @covers: MemoryEventStore::load_from — returns events at exact sequence boundary
#[test]
fn test_memory_event_store_load_from_exact_boundary_edge() {
    let store = MemoryEventStore::<ItemEvt>::new();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "x",
        events: vec![ItemEvt("x".into()), ItemEvt("x".into()), ItemEvt("x".into())],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    let events = futures::executor::block_on(
        store.load_from(EventStoreLoadFromRequest { aggregate_id: "x", from_sequence: 3 }),
    )
    .expect("load_from")
    .events;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].sequence, 3);
}
