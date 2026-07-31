#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{GaugeLookupRequest, GaugeSetRequest, StdObserveFactory};

// --- set ---

#[test]
fn test_set_positive_value_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry
        .gauge(GaugeLookupRequest {
            name: "queue.depth".to_string(),
        })
        .unwrap()
        .gauge;
    gauge.set(GaugeSetRequest { value: 100.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is a ZST");
}

#[test]
fn test_set_negative_value_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry
        .gauge(GaugeLookupRequest {
            name: "queue.depth".to_string(),
        })
        .unwrap()
        .gauge;
    gauge.set(GaugeSetRequest { value: -50.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is a ZST");
}

#[test]
fn test_set_zero_value_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry
        .gauge(GaugeLookupRequest {
            name: "queue.depth".to_string(),
        })
        .unwrap()
        .gauge;
    gauge.set(GaugeSetRequest { value: 0.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is a ZST");
}

#[test]
fn test_gauge_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry
        .gauge(GaugeLookupRequest {
            name: "g".to_string(),
        })
        .unwrap()
        .gauge;
    assert_send_sync(&gauge);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "gauge is Send+Sync ZST");
}
