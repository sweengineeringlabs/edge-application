//! SAF facade tests — `Counter` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::atomic::{AtomicU64, Ordering};

use edge_application_base::{Counter, IncrementRequest, IncrementResponse, ObserveError};

#[derive(Default)]
struct RecordingCounter {
    total: AtomicU64,
}
impl Counter for RecordingCounter {
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, ObserveError> {
        self.total.fetch_add(req.delta, Ordering::SeqCst);
        Ok(IncrementResponse)
    }
}

/// @covers: Counter::increment — recorded delta is applied
#[test]
fn test_increment_positive_delta_updates_total_happy() {
    let counter = RecordingCounter::default();
    counter
        .increment(IncrementRequest { delta: 5 })
        .expect("increment should succeed");
    assert_eq!(counter.total.load(Ordering::SeqCst), 5);
}

/// @covers: Counter::increment — zero delta is a no-op error boundary
#[test]
fn test_increment_zero_delta_leaves_total_unchanged_error() {
    let counter = RecordingCounter::default();
    counter
        .increment(IncrementRequest { delta: 0 })
        .expect("increment should succeed");
    assert_eq!(counter.total.load(Ordering::SeqCst), 0);
}

/// @covers: Counter::increment — repeated increments accumulate
#[test]
fn test_increment_repeated_calls_accumulate_edge() {
    let counter = RecordingCounter::default();
    counter.increment(IncrementRequest { delta: 3 }).unwrap();
    counter.increment(IncrementRequest { delta: 4 }).unwrap();
    assert_eq!(counter.total.load(Ordering::SeqCst), 7);
}
