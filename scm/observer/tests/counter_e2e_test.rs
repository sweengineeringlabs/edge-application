#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{CounterLookupRequest, IncrementRequest, StdObserveFactory};

// --- increment ---

#[test]
fn test_increment_positive_delta_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry
        .counter(CounterLookupRequest {
            name: "requests.total".to_string(),
        })
        .unwrap()
        .counter;
    counter.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*counter), 0);
}

#[test]
fn test_increment_max_value_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry
        .counter(CounterLookupRequest {
            name: "requests.total".to_string(),
        })
        .unwrap()
        .counter;
    counter
        .increment(IncrementRequest { delta: u64::MAX })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*counter), 0);
}

#[test]
fn test_increment_zero_delta_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry
        .counter(CounterLookupRequest {
            name: "requests.total".to_string(),
        })
        .unwrap()
        .counter;
    counter.increment(IncrementRequest { delta: 0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*counter), 0);
}

#[test]
fn test_counter_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry
        .counter(CounterLookupRequest {
            name: "c".to_string(),
        })
        .unwrap()
        .counter;
    assert_send_sync(&counter);
    assert_eq!(std::mem::size_of_val(&*counter), 0, "counter is Send+Sync ZST");
}
