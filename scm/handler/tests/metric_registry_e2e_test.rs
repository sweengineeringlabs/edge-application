//! SAF facade tests — `MetricRegistry` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    Counter, CounterLookupRequest, CounterLookupResponse, Gauge, GaugeLookupRequest,
    GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, Histogram, HistogramLookupRequest,
    HistogramLookupResponse, HistogramRecordRequest, HistogramRecordResponse, IncrementRequest,
    IncrementResponse, MetricRegistry, ObserveError,
};

struct StubCounter;
impl Counter for StubCounter {
    fn increment(&self, _req: IncrementRequest) -> Result<IncrementResponse, ObserveError> {
        Ok(IncrementResponse)
    }
}

struct StubHistogram;
impl Histogram for StubHistogram {
    fn record(&self, _req: HistogramRecordRequest) -> Result<HistogramRecordResponse, ObserveError> {
        Ok(HistogramRecordResponse)
    }
}

struct StubGauge;
impl Gauge for StubGauge {
    fn set(&self, _req: GaugeSetRequest) -> Result<GaugeSetResponse, ObserveError> {
        Ok(GaugeSetResponse)
    }
}

struct FakeRegistry;
impl MetricRegistry for FakeRegistry {
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, ObserveError> {
        if req.name.is_empty() {
            return Err(ObserveError::BackendUnavailable("empty name".into()));
        }
        Ok(CounterLookupResponse {
            counter: Box::new(StubCounter),
        })
    }
    fn histogram(
        &self,
        req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, ObserveError> {
        if req.name.is_empty() {
            return Err(ObserveError::BackendUnavailable("empty name".into()));
        }
        Ok(HistogramLookupResponse {
            histogram: Box::new(StubHistogram),
        })
    }
    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, ObserveError> {
        if req.name.is_empty() {
            return Err(ObserveError::BackendUnavailable("empty name".into()));
        }
        Ok(GaugeLookupResponse {
            gauge: Box::new(StubGauge),
        })
    }
}

/// @covers: MetricRegistry::counter — resolves a usable counter for a known name
#[test]
fn test_counter_known_name_returns_counter_happy() {
    let registry = FakeRegistry;
    let response = registry
        .counter(CounterLookupRequest { name: "requests".into() })
        .expect("counter should succeed");
    assert!(response.counter.increment(IncrementRequest { delta: 1 }).is_ok());
}

/// @covers: MetricRegistry::counter — empty name is rejected
#[test]
fn test_counter_empty_name_returns_err_error() {
    let registry = FakeRegistry;
    assert!(matches!(
        registry.counter(CounterLookupRequest { name: String::new() }),
        Err(ObserveError::BackendUnavailable(_))
    ));
}

/// @covers: MetricRegistry::counter — repeated lookups return independent instruments
#[test]
fn test_counter_repeated_lookups_return_independent_instruments_edge() {
    let registry = FakeRegistry;
    let a = registry.counter(CounterLookupRequest { name: "a".into() }).unwrap();
    let b = registry.counter(CounterLookupRequest { name: "b".into() }).unwrap();
    assert!(a.counter.increment(IncrementRequest { delta: 1 }).is_ok());
    assert!(b.counter.increment(IncrementRequest { delta: 1 }).is_ok());
}

/// @covers: MetricRegistry::histogram — resolves a usable histogram for a known name
#[test]
fn test_histogram_known_name_returns_histogram_happy() {
    let registry = FakeRegistry;
    let response = registry
        .histogram(HistogramLookupRequest { name: "latency".into() })
        .expect("histogram should succeed");
    assert!(response
        .histogram
        .record(HistogramRecordRequest { value: 1.0 })
        .is_ok());
}

/// @covers: MetricRegistry::histogram — empty name is rejected
#[test]
fn test_histogram_empty_name_returns_err_error() {
    let registry = FakeRegistry;
    assert!(matches!(
        registry.histogram(HistogramLookupRequest { name: String::new() }),
        Err(ObserveError::BackendUnavailable(_))
    ));
}

/// @covers: MetricRegistry::histogram — repeated lookups return independent instruments
#[test]
fn test_histogram_repeated_lookups_return_independent_instruments_edge() {
    let registry = FakeRegistry;
    let a = registry
        .histogram(HistogramLookupRequest { name: "a".into() })
        .unwrap();
    let b = registry
        .histogram(HistogramLookupRequest { name: "b".into() })
        .unwrap();
    assert!(a.histogram.record(HistogramRecordRequest { value: 1.0 }).is_ok());
    assert!(b.histogram.record(HistogramRecordRequest { value: 2.0 }).is_ok());
}

/// @covers: MetricRegistry::gauge — resolves a usable gauge for a known name
#[test]
fn test_gauge_known_name_returns_gauge_happy() {
    let registry = FakeRegistry;
    let response = registry
        .gauge(GaugeLookupRequest { name: "queue_depth".into() })
        .expect("gauge should succeed");
    assert!(response.gauge.set(GaugeSetRequest { value: 3.0 }).is_ok());
}

/// @covers: MetricRegistry::gauge — empty name is rejected
#[test]
fn test_gauge_empty_name_returns_err_error() {
    let registry = FakeRegistry;
    assert!(matches!(
        registry.gauge(GaugeLookupRequest { name: String::new() }),
        Err(ObserveError::BackendUnavailable(_))
    ));
}

/// @covers: MetricRegistry::gauge — repeated lookups return independent instruments
#[test]
fn test_gauge_repeated_lookups_return_independent_instruments_edge() {
    let registry = FakeRegistry;
    let a = registry.gauge(GaugeLookupRequest { name: "a".into() }).unwrap();
    let b = registry.gauge(GaugeLookupRequest { name: "b".into() }).unwrap();
    assert!(a.gauge.set(GaugeSetRequest { value: 1.0 }).is_ok());
    assert!(b.gauge.set(GaugeSetRequest { value: 2.0 }).is_ok());
}
