//! Integration tests for `MemoryProjection`.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_application_projection::{
    DomainEvent, MemoryProjection, Projection, ProjectionApplyRequest, ProjectionReadModelRequest,
};

#[derive(Clone)]
struct ItemEvt {
    count: usize,
}

impl DomainEvent for ItemEvt {
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "cart" })
    }
}

fn make(seed: usize) -> MemoryProjection<ItemEvt, usize, impl Fn(&mut usize, &ItemEvt) + Send + Sync> {
    MemoryProjection::new(seed, |total: &mut usize, e: &ItemEvt| *total += e.count)
}

fn read(p: &impl Projection<Event = ItemEvt, ReadModel = usize>) -> usize {
    *p.read_model(ProjectionReadModelRequest).expect("read_model should succeed").read_model
}

#[test]
fn test_new_projection_read_model_matches_initial_happy() {
    let p = make(5);
    assert_eq!(read(&p), 5);
}

#[test]
fn test_new_projection_with_zero_initial_is_zero_error() {
    let p = make(0);
    assert_eq!(read(&p), 0);
}

#[test]
fn test_projection_reducers_are_independent_edge() {
    let mut p1 = make(0);
    let p2 = MemoryProjection::new(0usize, |_n: &mut usize, _e: &ItemEvt| {});
    p1.apply(ProjectionApplyRequest { event: &ItemEvt { count: 10 } }).expect("apply should succeed");
    assert_eq!(read(&p1), 10);
    assert_eq!(read(&p2), 0);
}

#[test]
fn test_apply_then_read_model_returns_updated_value_happy() {
    let mut p = make(0);
    p.apply(ProjectionApplyRequest { event: &ItemEvt { count: 3 } }).expect("apply should succeed");
    p.apply(ProjectionApplyRequest { event: &ItemEvt { count: 7 } }).expect("apply should succeed");
    assert_eq!(read(&p), 10);
}
