//! SAF facade tests — `ObserverContext` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    CounterLookupRequest, CounterLookupResponse, DrainRequest, GaugeLookupRequest,
    GaugeLookupResponse, HandlerTracer, HistogramLookupRequest, HistogramLookupResponse,
    LogDrain, LogEmitRequest, LogEmitResponse, MetricRegistry, MetricsRequest, ObserveError,
    ObserverContext, SpanStartRequest, SpanStartResponse, TracerRequest,
};

struct StubTracer;
impl HandlerTracer for StubTracer {
    fn start_span(&self, _req: SpanStartRequest) -> Result<SpanStartResponse, ObserveError> {
        Err(ObserveError::NotInitialised)
    }
}

struct StubDrain;
impl LogDrain for StubDrain {
    fn emit(&self, _req: LogEmitRequest) -> Result<LogEmitResponse, ObserveError> {
        Ok(LogEmitResponse)
    }
}

struct StubRegistry;
impl MetricRegistry for StubRegistry {
    fn counter(&self, _req: CounterLookupRequest) -> Result<CounterLookupResponse, ObserveError> {
        Err(ObserveError::NotInitialised)
    }
    fn histogram(
        &self,
        _req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, ObserveError> {
        Err(ObserveError::NotInitialised)
    }
    fn gauge(&self, _req: GaugeLookupRequest) -> Result<GaugeLookupResponse, ObserveError> {
        Err(ObserveError::NotInitialised)
    }
}

struct FakeContext {
    tracer: StubTracer,
    drain: StubDrain,
    registry: StubRegistry,
}
impl ObserverContext for FakeContext {
    fn tracer(
        &self,
        _req: TracerRequest,
    ) -> Result<edge_application_handler::TracerResponse<'_>, ObserveError> {
        Ok(edge_application_handler::TracerResponse { tracer: &self.tracer })
    }
    fn drain(
        &self,
        _req: DrainRequest,
    ) -> Result<edge_application_handler::DrainResponse<'_>, ObserveError> {
        Ok(edge_application_handler::DrainResponse { drain: &self.drain })
    }
    fn metrics(
        &self,
        _req: MetricsRequest,
    ) -> Result<edge_application_handler::MetricsResponse<'_>, ObserveError> {
        Ok(edge_application_handler::MetricsResponse {
            metrics: &self.registry,
        })
    }
}

fn context() -> FakeContext {
    FakeContext {
        tracer: StubTracer,
        drain: StubDrain,
        registry: StubRegistry,
    }
}

/// @covers: ObserverContext::tracer — returns the active tracer
#[test]
fn test_tracer_returns_active_tracer_happy() {
    let ctx = context();
    let response = ctx.tracer(TracerRequest).expect("tracer should succeed");
    let result = response.tracer.start_span(SpanStartRequest {
        handler_id: "h".into(),
        operation: "op".into(),
    });
    match result {
        Err(err) => assert_eq!(err, ObserveError::NotInitialised),
        Ok(_) => panic!("stub tracer should always fail"),
    }
}

/// @covers: ObserverContext::tracer — propagates the tracer's own failure mode
#[test]
fn test_tracer_delegates_to_underlying_tracer_error() {
    let ctx = context();
    let response = ctx.tracer(TracerRequest).expect("tracer should succeed");
    assert!(response
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "h".into(),
            operation: "op".into(),
        })
        .is_err());
}

/// @covers: ObserverContext::tracer — repeated calls each resolve to a working tracer
#[test]
fn test_tracer_repeated_calls_each_resolve_edge() {
    let ctx = context();
    for _ in 0..2 {
        let response = ctx.tracer(TracerRequest).expect("tracer should succeed");
        let result = response.tracer.start_span(SpanStartRequest {
            handler_id: "h".into(),
            operation: "op".into(),
        });
        match result {
            Err(err) => assert_eq!(err, ObserveError::NotInitialised),
            Ok(_) => panic!("stub tracer should always fail"),
        }
    }
}

/// @covers: ObserverContext::drain — returns a usable log drain
#[test]
fn test_drain_returns_active_drain_happy() {
    let ctx = context();
    let response = ctx.drain(DrainRequest).expect("drain should succeed");
    assert!(response
        .drain
        .emit(LogEmitRequest {
            level: "INFO".into(),
            handler_id: "h".into(),
            message: "m".into(),
        })
        .is_ok());
}

/// @covers: ObserverContext::drain — empty message still forwards to the drain
#[test]
fn test_drain_emit_empty_message_still_forwards_error() {
    let ctx = context();
    let response = ctx.drain(DrainRequest).expect("drain should succeed");
    assert!(response
        .drain
        .emit(LogEmitRequest {
            level: "INFO".into(),
            handler_id: "h".into(),
            message: String::new(),
        })
        .is_ok());
}

/// @covers: ObserverContext::drain — repeated calls each resolve to a working drain
#[test]
fn test_drain_repeated_calls_each_resolve_edge() {
    let ctx = context();
    for i in 0..2 {
        let response = ctx.drain(DrainRequest).expect("drain should succeed");
        let result = response.drain.emit(LogEmitRequest {
            level: "INFO".into(),
            handler_id: "h".into(),
            message: format!("call-{i}"),
        });
        assert_eq!(result, Ok(LogEmitResponse));
    }
}

/// @covers: ObserverContext::metrics — returns the active metric registry
#[test]
fn test_metrics_returns_active_registry_happy() {
    let ctx = context();
    let response = ctx.metrics(MetricsRequest).expect("metrics should succeed");
    assert!(response
        .metrics
        .counter(CounterLookupRequest { name: "x".into() })
        .is_err());
}

/// @covers: ObserverContext::metrics — propagates the registry's own failure mode
#[test]
fn test_metrics_delegates_to_underlying_registry_error() {
    let ctx = context();
    let response = ctx.metrics(MetricsRequest).expect("metrics should succeed");
    assert_eq!(
        response.metrics.gauge(GaugeLookupRequest { name: "x".into() }).err(),
        Some(ObserveError::NotInitialised)
    );
}

/// @covers: ObserverContext::metrics — repeated calls each resolve to a working registry
#[test]
fn test_metrics_repeated_calls_each_resolve_edge() {
    let ctx = context();
    for _ in 0..2 {
        let response = ctx.metrics(MetricsRequest).expect("metrics should succeed");
        assert_eq!(
            response.metrics.counter(CounterLookupRequest { name: "x".into() }).err(),
            Some(ObserveError::NotInitialised)
        );
    }
}
