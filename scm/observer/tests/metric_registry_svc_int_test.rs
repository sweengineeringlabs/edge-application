#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, GaugeLookupRequest, GaugeSetRequest, HistogramLookupRequest,
    HistogramRecordRequest, IncrementRequest, MetricRegistry, StdObserveFactory,
    METRIC_REGISTRY_SVC,
};

#[test]
fn test_noop_metric_registry_svc_counter_increments_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry
        .counter(CounterLookupRequest {
            name: "http.requests".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 1 })
        .unwrap();
    // Counter was created and incremented without error.
    assert_eq!(std::mem::size_of_val(&*registry), 0);
}

#[test]
fn test_noop_metric_registry_svc_empty_metric_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry
        .gauge(GaugeLookupRequest {
            name: "".to_string(),
        })
        .unwrap()
        .gauge
        .set(GaugeSetRequest { value: -1.0 })
        .unwrap();
    // Empty metric name is handled without error.
    assert_eq!(std::mem::size_of_val(&*registry), 0);
}

#[test]
fn test_noop_metric_registry_svc_all_instrument_types_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry
        .counter(CounterLookupRequest {
            name: "c".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 100 })
        .unwrap();
    registry
        .gauge(GaugeLookupRequest {
            name: "g".to_string(),
        })
        .unwrap()
        .gauge
        .set(GaugeSetRequest { value: 3.5 })
        .unwrap();
    registry
        .histogram(HistogramLookupRequest {
            name: "h".to_string(),
        })
        .unwrap()
        .histogram
        .record(HistogramRecordRequest { value: 0.001 })
        .unwrap();
    // All instruments created without error.
    assert_eq!(std::mem::size_of_val(&*registry), 0);
}

#[test]
fn test_metric_registry_svc_key_namespaced_happy() {
    assert!(METRIC_REGISTRY_SVC.starts_with("edge."));
}

#[test]
fn test_metric_registry_svc_returns_dyn_trait_object() {
    let _registry: Box<dyn MetricRegistry> = StdObserveFactory::noop_metric_registry();
}
