//! Layer-level coverage for the small request/response value types declared under
//! `api/context/command/` and `api/context/observe/` that have no dedicated per-type
//! test file (SEA layer test coverage, `sea_layer_test_coverage`). Each test constructs
//! the type through the crate's public API and asserts on its real shape, field values,
//! or ZST-ness.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_base::{
    CommandError, Counter, CounterLookupRequest, CounterLookupResponse, DrainRequest,
    DrainResponse, Gauge, GaugeLookupRequest, GaugeLookupResponse, GaugeSetRequest,
    GaugeSetResponse, HandlerTracer, Histogram, HistogramLookupRequest, HistogramLookupResponse,
    HistogramRecordRequest, HistogramRecordResponse, IncrementRequest, IncrementResponse,
    LogDrain, LogEmitRequest, LogEmitResponse, MetricRegistry, MetricsRequest, MetricsResponse,
    ObserveError, Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse, SpanStartRequest, SpanStartResponse, TracerRequest, TracerResponse,
};

// --- command domain DTOs ---

/// @covers: ExecutionRequest (as CommandExecutionRequest)
#[test]
fn test_command_execution_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<edge_application_base::CommandExecutionRequest>(), 0);
}

/// @covers: NameRequest (as CommandNameRequest)
#[test]
fn test_command_name_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<edge_application_base::CommandNameRequest>(), 0);
}

/// @covers: NameResponse (as CommandNameResponse)
#[test]
fn test_command_name_response_holds_name_happy() {
    let r = edge_application_base::CommandNameResponse {
        name: "create-order".to_string(),
    };
    assert_eq!(r.name, "create-order");
}

/// @covers: CommandError — Display includes the wrapped message
#[test]
fn test_command_error_display_includes_message_happy() {
    let e = CommandError::InvalidInput("bad id".into());
    assert!(e.to_string().contains("bad id"));
}

/// @covers: CommandError — variants format distinctly
#[test]
fn test_command_error_variants_format_distinctly_edge() {
    let nf = CommandError::NotFound("x".into()).to_string();
    let internal = CommandError::Internal("y".into()).to_string();
    assert!(nf.contains("not found"));
    assert!(internal.contains("internal"));
    assert_ne!(nf, internal);
}

// --- observe domain: instrument lookup requests/responses ---

struct RelayCounter;
impl Counter for RelayCounter {
    fn increment(&self, _req: IncrementRequest) -> Result<IncrementResponse, ObserveError> {
        Ok(IncrementResponse)
    }
}

struct RelayGauge;
impl Gauge for RelayGauge {
    fn set(&self, _req: GaugeSetRequest) -> Result<GaugeSetResponse, ObserveError> {
        Ok(GaugeSetResponse)
    }
}

struct RelayHistogram;
impl Histogram for RelayHistogram {
    fn record(&self, _req: HistogramRecordRequest) -> Result<HistogramRecordResponse, ObserveError> {
        Ok(HistogramRecordResponse)
    }
}

struct RelaySpan;
impl Span for RelaySpan {
    fn record(&self, _req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, ObserveError> {
        Ok(SpanAnnotationResponse)
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, ObserveError> {
        Ok(SpanFinishResponse)
    }
}

struct RelayTracer;
impl HandlerTracer for RelayTracer {
    fn start_span(&self, _req: SpanStartRequest) -> Result<SpanStartResponse, ObserveError> {
        Ok(SpanStartResponse {
            span: Box::new(RelaySpan),
        })
    }
}

struct RelayDrain;
impl LogDrain for RelayDrain {
    fn emit(&self, _req: LogEmitRequest) -> Result<LogEmitResponse, ObserveError> {
        Ok(LogEmitResponse)
    }
}

struct RelayRegistry;
impl MetricRegistry for RelayRegistry {
    fn counter(&self, _req: CounterLookupRequest) -> Result<CounterLookupResponse, ObserveError> {
        Ok(CounterLookupResponse {
            counter: Box::new(RelayCounter),
        })
    }
    fn histogram(
        &self,
        _req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, ObserveError> {
        Ok(HistogramLookupResponse {
            histogram: Box::new(RelayHistogram),
        })
    }
    fn gauge(&self, _req: GaugeLookupRequest) -> Result<GaugeLookupResponse, ObserveError> {
        Ok(GaugeLookupResponse {
            gauge: Box::new(RelayGauge),
        })
    }
}

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
    let r = CounterLookupResponse {
        counter: Box::new(RelayCounter),
    };
    assert_eq!(r.counter.increment(IncrementRequest { delta: 1 }), Ok(IncrementResponse));
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
    let r = GaugeLookupResponse {
        gauge: Box::new(RelayGauge),
    };
    assert_eq!(r.gauge.set(GaugeSetRequest { value: 1.0 }), Ok(GaugeSetResponse));
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
    let r = HistogramLookupResponse {
        histogram: Box::new(RelayHistogram),
    };
    assert_eq!(
        r.histogram.record(HistogramRecordRequest { value: 12.5 }),
        Ok(HistogramRecordResponse)
    );
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
}

/// @covers: SpanFinishRequest
#[test]
fn test_span_finish_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SpanFinishRequest>(), 0);
}

/// @covers: SpanFinishResponse
#[test]
fn test_span_finish_response_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SpanFinishResponse>(), 0);
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
    let r = SpanStartResponse {
        span: Box::new(RelaySpan),
    };
    assert_eq!(
        r.span.record(SpanAnnotationRequest {
            key: "k".to_string(),
            value: "v".to_string(),
        }),
        Ok(SpanAnnotationResponse)
    );
}

/// @covers: DrainRequest
#[test]
fn test_drain_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<DrainRequest>(), 0);
}

/// @covers: DrainResponse
#[test]
fn test_drain_response_holds_log_drain_happy() {
    let drain = RelayDrain;
    let r = DrainResponse { drain: &drain };
    assert_eq!(
        r.drain.emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        }),
        Ok(LogEmitResponse)
    );
}

/// @covers: MetricsRequest
#[test]
fn test_metrics_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<MetricsRequest>(), 0);
}

/// @covers: MetricsResponse
#[test]
fn test_metrics_response_holds_metric_registry_happy() {
    let registry = RelayRegistry;
    let r = MetricsResponse { metrics: &registry };
    assert_eq!(
        r.metrics.gauge(GaugeLookupRequest {
            name: "g".to_string(),
        }).map(|resp| resp.gauge.set(GaugeSetRequest { value: 1.0 })),
        Ok(Ok(GaugeSetResponse))
    );
}

/// @covers: TracerRequest
#[test]
fn test_tracer_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<TracerRequest>(), 0);
}

/// @covers: TracerResponse
#[test]
fn test_tracer_response_holds_handler_tracer_happy() {
    let tracer = RelayTracer;
    let r = TracerResponse { tracer: &tracer };
    let span_result = r.tracer.start_span(SpanStartRequest {
        handler_id: "h".to_string(),
        operation: "op".to_string(),
    });
    assert_eq!(
        span_result.map(|resp| resp.span.finish(SpanFinishRequest)),
        Ok(Ok(SpanFinishResponse))
    );
}

/// @covers: ObserveError — variants are distinguishable
#[test]
fn test_observe_error_variants_are_distinguishable_happy() {
    assert_ne!(
        ObserveError::NotInitialised,
        ObserveError::BackendUnavailable("x".into())
    );
}

/// @covers: ObserveError — BackendUnavailable carries the reason
#[test]
fn test_observe_error_backend_unavailable_carries_reason_edge() {
    let e = ObserveError::BackendUnavailable("no backend".into());
    assert_eq!(e, ObserveError::BackendUnavailable("no backend".into()));
}

/// @covers: ObserveError — Display includes the wrapped message
#[test]
fn test_observe_error_display_backend_unavailable_includes_message_happy() {
    let e = ObserveError::BackendUnavailable("timeout".to_string());
    assert_eq!(e.to_string(), "observe backend unavailable: timeout");
}

/// @covers: ObserveError — Display for NotInitialised has a fixed message
#[test]
fn test_observe_error_display_not_initialised_message_error() {
    assert_eq!(
        ObserveError::NotInitialised.to_string(),
        "observe backend not initialised"
    );
}
