//! SAF tests — `Projection` trait via `MemoryProjection`.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_application_projection::{
    DomainEvent, MemoryProjection, Projection, ProjectionApplyRequest, ProjectionError,
    ProjectionReadModelRequest, TryDrainRequest,
};

#[derive(Clone)]
struct BalanceEvt {
    delta: i64,
}

impl DomainEvent for BalanceEvt {
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "account-1" })
    }
}

fn make_balance(seed: i64) -> impl Projection<Event = BalanceEvt, ReadModel = i64> {
    MemoryProjection::new(seed, |total: &mut i64, e: &BalanceEvt| *total += e.delta)
}

fn evt(delta: i64) -> BalanceEvt {
    BalanceEvt { delta }
}

fn read(p: &impl Projection<Event = BalanceEvt, ReadModel = i64>) -> i64 {
    *p.read_model(ProjectionReadModelRequest).expect("read_model should succeed").read_model
}

/// @covers: apply
#[test]
fn test_apply_single_event_increments_read_model_happy() {
    let mut p = make_balance(0);
    p.apply(ProjectionApplyRequest { event: &evt(10) }).expect("apply should succeed");
    assert_eq!(read(&p), 10);
}

/// @covers: apply
#[test]
fn test_apply_zero_delta_leaves_read_model_unchanged_error() {
    let mut p = make_balance(7);
    p.apply(ProjectionApplyRequest { event: &evt(0) }).expect("apply should succeed");
    assert_eq!(read(&p), 7);
}

/// @covers: apply
#[test]
fn test_apply_many_events_accumulate_all_deltas_edge() {
    let mut p = make_balance(0);
    for i in 1..=10i64 {
        p.apply(ProjectionApplyRequest { event: &evt(i) }).expect("apply should succeed");
    }
    assert_eq!(read(&p), 55);
}

/// @covers: read_model
#[test]
fn test_read_model_initial_value_matches_seed_happy() {
    let p = make_balance(42);
    assert_eq!(read(&p), 42);
}

/// @covers: read_model
#[test]
fn test_read_model_before_events_is_seed_value_error() {
    let p = make_balance(0);
    assert_eq!(read(&p), 0);
}

/// @covers: read_model
#[test]
fn test_read_model_reflects_all_applied_events_edge() {
    let mut p = make_balance(100);
    p.apply(ProjectionApplyRequest { event: &evt(-50) }).expect("apply should succeed");
    p.apply(ProjectionApplyRequest { event: &evt(20) }).expect("apply should succeed");
    assert_eq!(read(&p), 70);
}

fn make_count(seed: u32) -> MemoryProjection<BalanceEvt, u32, impl Fn(&mut u32, &BalanceEvt) + Send + Sync> {
    MemoryProjection::new(seed, |count: &mut u32, _e: &BalanceEvt| *count += 1)
}

/// @covers: apply
#[test]
fn test_apply_counts_each_event_regardless_of_payload_edge() {
    let mut p = make_count(0);
    p.apply(ProjectionApplyRequest { event: &evt(0) }).expect("apply should succeed");
    p.apply(ProjectionApplyRequest { event: &evt(999) }).expect("apply should succeed");
    assert_eq!(*p.read_model(ProjectionReadModelRequest).expect("read_model").read_model, 2);
}

/// @covers: try_drain
#[test]
fn test_try_drain_events_slice_returns_count_happy() {
    let mut p = make_balance(0);
    let events = [evt(3), evt(4)];
    let response = p.try_drain(TryDrainRequest { events: &events }).expect("try_drain should succeed");
    assert_eq!(response.count, 2);
    assert_eq!(read(&p), 7);
}

/// @covers: try_drain
#[test]
fn test_try_drain_empty_slice_returns_empty_stream_error() {
    let mut p = make_balance(0);
    let err = p.try_drain(TryDrainRequest { events: &[] }).unwrap_err();
    assert_eq!(err, ProjectionError::EmptyStream);
}

/// @covers: try_drain
#[test]
fn test_try_drain_single_event_returns_one_edge() {
    let mut p = make_balance(0);
    let response = p.try_drain(TryDrainRequest { events: &[evt(1)] }).expect("try_drain should succeed");
    assert_eq!(response.count, 1);
    assert_eq!(read(&p), 1);
}
