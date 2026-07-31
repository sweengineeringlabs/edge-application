#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{HistogramLookupRequest, HistogramRecordRequest, StdObserveFactory};

// --- record (Histogram) ---

#[test]
fn test_record_positive_value_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry
        .histogram(HistogramLookupRequest {
            name: "handler.latency_ms".to_string(),
        })
        .unwrap()
        .histogram;
    hist.record(HistogramRecordRequest { value: 42.5 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_record_negative_value_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry
        .histogram(HistogramLookupRequest {
            name: "handler.latency_ms".to_string(),
        })
        .unwrap()
        .histogram;
    hist.record(HistogramRecordRequest { value: -1.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_record_zero_value_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry
        .histogram(HistogramLookupRequest {
            name: "handler.latency_ms".to_string(),
        })
        .unwrap()
        .histogram;
    hist.record(HistogramRecordRequest { value: 0.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_histogram_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry
        .histogram(HistogramLookupRequest {
            name: "h".to_string(),
        })
        .unwrap()
        .histogram;
    assert_send_sync(&hist);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}
