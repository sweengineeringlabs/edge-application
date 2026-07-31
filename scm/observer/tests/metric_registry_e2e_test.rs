#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, GaugeLookupRequest, GaugeSetRequest, HistogramLookupRequest,
    HistogramRecordRequest, IncrementRequest, MetricRegistry, StdObserveFactory,
};

// --- counter (MetricRegistry) ---

#[test]
fn test_counter_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry
        .counter(CounterLookupRequest {
            name: "requests.total".to_string(),
        })
        .unwrap()
        .counter;
    counter.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*counter), 0, "noop counter is ZST");
}

#[test]
fn test_counter_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry
        .counter(CounterLookupRequest {
            name: "".to_string(),
        })
        .unwrap()
        .counter;
    counter.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*counter), 0, "noop counter is ZST");
}

#[test]
fn test_counter_multiple_instruments_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let a = registry
        .counter(CounterLookupRequest { name: "a".to_string() })
        .unwrap()
        .counter;
    let b = registry
        .counter(CounterLookupRequest { name: "b".to_string() })
        .unwrap()
        .counter;
    a.increment(IncrementRequest { delta: 1 }).unwrap();
    b.increment(IncrementRequest { delta: 2 }).unwrap();
    a.increment(IncrementRequest { delta: 3 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop counter is ZST");
}

// --- histogram (MetricRegistry) ---

#[test]
fn test_histogram_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry
        .histogram(HistogramLookupRequest {
            name: "latency_ms".to_string(),
        })
        .unwrap()
        .histogram;
    hist.record(HistogramRecordRequest { value: 10.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_histogram_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry
        .histogram(HistogramLookupRequest {
            name: "".to_string(),
        })
        .unwrap()
        .histogram;
    hist.record(HistogramRecordRequest { value: 1.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_histogram_same_name_multiple_times_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let h1 = registry
        .histogram(HistogramLookupRequest { name: "h".to_string() })
        .unwrap()
        .histogram;
    let h2 = registry
        .histogram(HistogramLookupRequest { name: "h".to_string() })
        .unwrap()
        .histogram;
    h1.record(HistogramRecordRequest { value: 1.0 }).unwrap();
    h2.record(HistogramRecordRequest { value: 2.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*h1), 0, "noop histogram is ZST");
    assert_eq!(std::mem::size_of_val(&*h2), 0, "noop histogram is ZST");
}

// --- gauge (MetricRegistry) ---

#[test]
fn test_gauge_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry
        .gauge(GaugeLookupRequest {
            name: "queue.depth".to_string(),
        })
        .unwrap()
        .gauge;
    gauge.set(GaugeSetRequest { value: 7.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is ZST");
}

#[test]
fn test_gauge_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry
        .gauge(GaugeLookupRequest {
            name: "".to_string(),
        })
        .unwrap()
        .gauge;
    gauge.set(GaugeSetRequest { value: 0.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is ZST");
}

#[test]
fn test_gauge_all_instruments_together_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let c = registry
        .counter(CounterLookupRequest { name: "c".to_string() })
        .unwrap()
        .counter;
    let h = registry
        .histogram(HistogramLookupRequest { name: "h".to_string() })
        .unwrap()
        .histogram;
    let g = registry
        .gauge(GaugeLookupRequest { name: "g".to_string() })
        .unwrap()
        .gauge;
    c.increment(IncrementRequest { delta: 1 }).unwrap();
    h.record(HistogramRecordRequest { value: 1.0 }).unwrap();
    g.set(GaugeSetRequest { value: 1.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_metric_registry_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    assert_send_sync(&registry);
    assert_eq!(std::mem::size_of_val(&*registry), 0, "noop metric registry is ZST");
}

#[test]
fn test_metric_registry_returns_dyn_trait_object() {
    let registry: Box<dyn MetricRegistry> = StdObserveFactory::noop_metric_registry();
    let c = registry
        .counter(CounterLookupRequest {
            name: "total".to_string(),
        })
        .unwrap()
        .counter;
    c.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}
