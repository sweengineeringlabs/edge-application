//! Layer-level coverage for the small request/response value types declared under
//! `api/observe/dto/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape, field values, or ZST-ness.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    BootstrapNameRequest, BootstrapNameResponse, Counter, CounterLookupRequest,
    CounterLookupResponse, DrainRequest, DrainResponse, Gauge, GaugeLookupRequest,
    GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, HandlerTracer,
    HandlerTracerBuildRequest, HandlerTracerBuildResponse, Histogram, HistogramLookupRequest,
    HistogramLookupResponse, HistogramRecordRequest, HistogramRecordResponse, IncrementRequest,
    IncrementResponse, LogDrain, LogDrainBuildRequest, LogDrainBuildResponse, LogEmitRequest,
    LogEmitResponse, MetricRegistry, MetricRegistryBuildRequest, MetricRegistryBuildResponse,
    MetricsRequest, MetricsResponse, NoopObserve, Span, SpanAnnotationRequest,
    SpanAnnotationResponse, SpanFinishRequest, SpanFinishResponse, SpanStartRequest,
    SpanStartResponse, StdObserveFactory, TracerRequest, TracerResponse, ValidationRequest,
    ValidationResponse,
};

// --- zero-sized markers ---

/// @covers: BootstrapNameRequest
#[test]
fn test_bootstrap_name_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<BootstrapNameRequest>(), 0);
    let _ = BootstrapNameRequest;
}

/// @covers: BootstrapNameResponse
#[test]
fn test_bootstrap_name_response_holds_name_happy() {
    let r = BootstrapNameResponse { name: "svc" };
    assert_eq!(r.name, "svc");
}

/// @covers: DrainRequest
#[test]
fn test_drain_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<DrainRequest>(), 0);
    let _ = DrainRequest;
}

/// @covers: DrainResponse
#[test]
fn test_drain_response_holds_log_drain_happy() {
    let drain = StdObserveFactory::build_noop_log_drain();
    let r = DrainResponse { drain: &*drain };
    r.drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
}

/// @covers: HandlerTracerBuildRequest
#[test]
fn test_handler_tracer_build_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<HandlerTracerBuildRequest>(), 0);
    let _ = HandlerTracerBuildRequest;
}

/// @covers: HandlerTracerBuildResponse
#[test]
fn test_handler_tracer_build_response_holds_tracer_happy() {
    let tracer: Box<dyn HandlerTracer> = StdObserveFactory::build_noop_handler_tracer();
    let r = HandlerTracerBuildResponse { tracer };
    let span = r
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    assert_eq!(std::mem::size_of_val(&*span), 0);
}

/// @covers: LogDrainBuildRequest
#[test]
fn test_log_drain_build_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<LogDrainBuildRequest>(), 0);
    let _ = LogDrainBuildRequest;
}

/// @covers: LogDrainBuildResponse
#[test]
fn test_log_drain_build_response_holds_log_drain_happy() {
    let log_drain: Box<dyn LogDrain> = StdObserveFactory::build_noop_log_drain();
    let r = LogDrainBuildResponse { log_drain };
    r.log_drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
}

/// @covers: MetricRegistryBuildRequest
#[test]
fn test_metric_registry_build_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<MetricRegistryBuildRequest>(), 0);
    let _ = MetricRegistryBuildRequest;
}

/// @covers: MetricRegistryBuildResponse
#[test]
fn test_metric_registry_build_response_holds_registry_happy() {
    let metric_registry: Box<dyn MetricRegistry> = StdObserveFactory::build_noop_metric_registry();
    let r = MetricRegistryBuildResponse { metric_registry };
    r.metric_registry
        .counter(CounterLookupRequest {
            name: "c".to_string(),
        })
        .unwrap();
}

/// @covers: MetricsRequest
#[test]
fn test_metrics_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<MetricsRequest>(), 0);
    let _ = MetricsRequest;
}

/// @covers: MetricsResponse
#[test]
fn test_metrics_response_holds_metric_registry_happy() {
    let registry = StdObserveFactory::build_noop_metric_registry();
    let r = MetricsResponse { metrics: &*registry };
    r.metrics
        .gauge(GaugeLookupRequest {
            name: "g".to_string(),
        })
        .unwrap();
}

/// @covers: SpanFinishRequest
#[test]
fn test_span_finish_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SpanFinishRequest>(), 0);
    let _ = SpanFinishRequest;
}

/// @covers: SpanFinishResponse
#[test]
fn test_span_finish_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SpanFinishResponse>(), 0);
    let _ = SpanFinishResponse;
}

/// @covers: TracerRequest
#[test]
fn test_tracer_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<TracerRequest>(), 0);
    let _ = TracerRequest;
}

/// @covers: TracerResponse
#[test]
fn test_tracer_response_holds_handler_tracer_happy() {
    let tracer = StdObserveFactory::build_noop_handler_tracer();
    let r = TracerResponse { tracer: &*tracer };
    r.tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap();
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ValidationRequest>(), 0);
    let _ = ValidationRequest;
}

/// @covers: ValidationResponse
#[test]
fn test_validation_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ValidationResponse>(), 0);
    let _ = ValidationResponse;
}

// --- instrument lookup requests/responses ---

/// @covers: CounterLookupRequest
#[test]
fn test_counter_lookup_request_holds_name_happy() {
    let r = CounterLookupRequest {
        name: "requests".to_string(),
    };
    assert_eq!(r.name, "requests");
}

/// @covers: CounterLookupResponse
#[test]
fn test_counter_lookup_response_holds_counter_happy() {
    let counter: Box<dyn Counter> = StdObserveFactory::build_noop_counter();
    let r = CounterLookupResponse { counter };
    r.counter.increment(IncrementRequest { delta: 1 }).unwrap();
}

/// @covers: GaugeLookupRequest
#[test]
fn test_gauge_lookup_request_holds_name_happy() {
    let r = GaugeLookupRequest {
        name: "queue_depth".to_string(),
    };
    assert_eq!(r.name, "queue_depth");
}

/// @covers: GaugeLookupResponse
#[test]
fn test_gauge_lookup_response_holds_gauge_happy() {
    let gauge: Box<dyn Gauge> = StdObserveFactory::build_noop_gauge();
    let r = GaugeLookupResponse { gauge };
    r.gauge.set(GaugeSetRequest { value: 1.0 }).unwrap();
}

/// @covers: GaugeSetRequest
#[test]
fn test_gauge_set_request_holds_value_happy() {
    let r = GaugeSetRequest { value: 4.0 };
    assert_eq!(r.value, 4.0);
}

/// @covers: GaugeSetResponse
#[test]
fn test_gauge_set_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<GaugeSetResponse>(), 0);
    let _ = GaugeSetResponse;
}

/// @covers: HistogramLookupRequest
#[test]
fn test_histogram_lookup_request_holds_name_happy() {
    let r = HistogramLookupRequest {
        name: "latency_ms".to_string(),
    };
    assert_eq!(r.name, "latency_ms");
}

/// @covers: HistogramLookupResponse
#[test]
fn test_histogram_lookup_response_holds_histogram_happy() {
    let histogram: Box<dyn Histogram> = StdObserveFactory::build_noop_histogram();
    let r = HistogramLookupResponse { histogram };
    r.histogram
        .record(HistogramRecordRequest { value: 12.5 })
        .unwrap();
}

/// @covers: HistogramRecordRequest
#[test]
fn test_histogram_record_request_holds_value_happy() {
    let r = HistogramRecordRequest { value: 12.5 };
    assert_eq!(r.value, 12.5);
}

/// @covers: HistogramRecordResponse
#[test]
fn test_histogram_record_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<HistogramRecordResponse>(), 0);
    let _ = HistogramRecordResponse;
}

/// @covers: IncrementRequest
#[test]
fn test_increment_request_holds_delta_happy() {
    let r = IncrementRequest { delta: 5 };
    assert_eq!(r.delta, 5);
}

/// @covers: IncrementResponse
#[test]
fn test_increment_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<IncrementResponse>(), 0);
    let _ = IncrementResponse;
}

/// @covers: LogEmitRequest
#[test]
fn test_log_emit_request_holds_fields_happy() {
    let r = LogEmitRequest {
        level: "INFO".to_string(),
        handler_id: "h".to_string(),
        message: "started".to_string(),
    };
    assert_eq!(r.level, "INFO");
    assert_eq!(r.handler_id, "h");
    assert_eq!(r.message, "started");
}

/// @covers: LogEmitResponse
#[test]
fn test_log_emit_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<LogEmitResponse>(), 0);
    let _ = LogEmitResponse;
}

/// @covers: SpanAnnotationRequest
#[test]
fn test_span_annotation_request_holds_key_value_happy() {
    let r = SpanAnnotationRequest {
        key: "db.rows".to_string(),
        value: "42".to_string(),
    };
    assert_eq!(r.key, "db.rows");
    assert_eq!(r.value, "42");
}

/// @covers: SpanAnnotationResponse
#[test]
fn test_span_annotation_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SpanAnnotationResponse>(), 0);
    let _ = SpanAnnotationResponse;
}

/// @covers: SpanStartRequest
#[test]
fn test_span_start_request_holds_fields_happy() {
    let r = SpanStartRequest {
        handler_id: "h".to_string(),
        operation: "op".to_string(),
    };
    assert_eq!(r.handler_id, "h");
    assert_eq!(r.operation, "op");
}

/// @covers: SpanStartResponse
#[test]
fn test_span_start_response_holds_span_happy() {
    let span: Box<dyn Span> = StdObserveFactory::build_noop_span();
    let r = SpanStartResponse { span };
    r.span
        .record(SpanAnnotationRequest {
            key: "k".to_string(),
            value: "v".to_string(),
        })
        .unwrap();
}
