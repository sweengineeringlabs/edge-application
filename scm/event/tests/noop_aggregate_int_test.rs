//! Integration tests for `NoopAggregate`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{Aggregate, AggregateApplyRequest, AggregateIdentityRequest, NoopAggregate, NoopDomainEvent};

/// @covers: NoopAggregate::id — returns empty string before any apply
#[test]
fn test_id_noop_aggregate_before_apply_returns_empty_happy() {
    let agg = NoopAggregate;
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}

/// @covers: NoopAggregate::apply — is a no-op; id stays empty after apply
#[test]
fn test_apply_noop_aggregate_after_apply_id_unchanged_happy() {
    let mut agg = NoopAggregate;
    agg.apply(AggregateApplyRequest { event: &NoopDomainEvent }).unwrap();
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}

/// @covers: NoopAggregate — Default constructs without panic
#[test]
fn test_default_noop_aggregate_constructs_edge() {
    let _agg = NoopAggregate;
    // Verify default constructs a valid aggregate
    assert_eq!(_agg.id(AggregateIdentityRequest).unwrap().id, "");
}

/// @covers: NoopAggregate::apply — repeated apply calls don't panic
#[test]
fn test_apply_noop_aggregate_repeated_calls_do_not_panic_edge() {
    let mut agg = NoopAggregate;
    for _ in 0..5 {
        agg.apply(AggregateApplyRequest { event: &NoopDomainEvent }).unwrap();
    }
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}

/// @covers: NoopAggregate — satisfies Aggregate trait bounds (Send + Sync)
#[test]
fn test_noop_aggregate_satisfies_send_sync_bounds_happy() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NoopAggregate>();
    // Verify NoopAggregate is actually a unit type (zero-sized)
    assert_eq!(std::mem::size_of::<NoopAggregate>(), 0);
}
