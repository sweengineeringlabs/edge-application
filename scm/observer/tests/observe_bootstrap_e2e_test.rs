#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, GaugeLookupRequest, GaugeSetRequest, HandlerTracerBuildRequest,
    HistogramLookupRequest, HistogramRecordRequest, IncrementRequest, LogDrainBuildRequest,
    LogEmitRequest, MetricRegistryBuildRequest, ObserveBootstrap,
    SpanAnnotationRequest, SpanFinishRequest, SpanStartRequest, StdObserveFactory,
    ValidationRequest, ValidationResponse,
};

// --- std_factory ---

#[test]
fn test_std_factory_returns_usable_factory_happy() {
    let factory = StdObserveFactory::std_factory();
    let tracer = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "factory produces ZST spans");
}

#[test]
fn test_std_factory_called_multiple_times_error() {
    let a = StdObserveFactory::std_factory();
    let b = StdObserveFactory::std_factory();
    let tracer = a
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    // `b` is a separate factory instance; ensure it is also usable.
    let _ = b.build_handler_tracer(HandlerTracerBuildRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "factories produce same ZST");
}

#[test]
fn test_std_factory_all_three_primitives_edge() {
    let factory = StdObserveFactory::std_factory();
    let tracer = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let registry = factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry;
    let drain = factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain;
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "all primitives are ZST");
    assert_eq!(std::mem::size_of_val(&*registry), 0, "registry is ZST");
    assert_eq!(std::mem::size_of_val(&*drain), 0, "drain is ZST");
}

// --- validate ---

#[test]
fn test_validate_initialized_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let result = factory.validate(ValidationRequest);
    assert_eq!(result, Ok(ValidationResponse), "factory.validate() should return Ok");
}

#[test]
fn test_validate_std_factory_returns_ok_error() {
    let factory = StdObserveFactory;
    let result = factory.validate(ValidationRequest);
    assert_eq!(
        result,
        Ok(ValidationResponse),
        "StdObserveFactory.validate() should return Ok"
    );
}

#[test]
fn test_validate_called_multiple_times_edge() {
    let factory = StdObserveFactory::create_factory();
    let first = factory.validate(ValidationRequest);
    let second = factory.validate(ValidationRequest);
    assert_eq!(first, Ok(ValidationResponse), "first validate should succeed");
    assert_eq!(second, Ok(ValidationResponse), "second validate should also succeed");
}

// --- build_handler_tracer ---

#[test]
fn test_build_handler_tracer_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let tracer = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "tracer produces ZST spans");
}

#[test]
fn test_build_handler_tracer_uninitialized_error() {
    let factory = StdObserveFactory;
    let tracer = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "".to_string(),
            operation: "".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "uninitialized factory produces ZST spans");
}

#[test]
fn test_build_handler_tracer_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let t1 = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let t2 = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let s1 = t1
        .start_span(SpanStartRequest {
            handler_id: "a".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    let s2 = t2
        .start_span(SpanStartRequest {
            handler_id: "b".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    s1.finish(SpanFinishRequest).unwrap();
    s2.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*s1), 0, "multiple tracers produce ZST spans");
}

// --- build_metric_registry ---

#[test]
fn test_build_metric_registry_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let registry = factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry;
    let counter = registry
        .counter(CounterLookupRequest { name: "c".to_string() })
        .unwrap()
        .counter;
    counter.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*counter), 0, "registry produces ZST counters");
}

#[test]
fn test_build_metric_registry_uninitialized_error() {
    let factory = StdObserveFactory;
    let registry = factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry;
    let gauge = registry
        .gauge(GaugeLookupRequest { name: "g".to_string() })
        .unwrap()
        .gauge;
    gauge.set(GaugeSetRequest { value: 0.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "uninitialized factory produces ZST gauges");
}

#[test]
fn test_build_metric_registry_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let r1 = factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry;
    let r2 = factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry;
    let c1 = r1
        .counter(CounterLookupRequest { name: "a".to_string() })
        .unwrap()
        .counter;
    let c2 = r2
        .counter(CounterLookupRequest { name: "b".to_string() })
        .unwrap()
        .counter;
    c1.increment(IncrementRequest { delta: 1 }).unwrap();
    c2.increment(IncrementRequest { delta: 2 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c1), 0, "multiple registries produce ZST instruments");
}

// --- build_log_drain ---

#[test]
fn test_build_log_drain_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let drain = factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain;
    drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "factory produces ZST drain");
}

#[test]
fn test_build_log_drain_uninitialized_error() {
    let factory = StdObserveFactory;
    let drain = factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain;
    drain
        .emit(LogEmitRequest {
            level: "ERROR".to_string(),
            handler_id: "h".to_string(),
            message: "err".to_string(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "uninitialized factory produces ZST drain");
}

#[test]
fn test_build_log_drain_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let d1 = factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain;
    let d2 = factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain;
    d1.emit(LogEmitRequest {
        level: "INFO".to_string(),
        handler_id: "a".to_string(),
        message: "1".to_string(),
    })
    .unwrap();
    d2.emit(LogEmitRequest {
        level: "WARN".to_string(),
        handler_id: "b".to_string(),
        message: "2".to_string(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*d1), 0, "multiple drains are ZST");
}

#[test]
fn test_observe_factory_all_three_primitives_independently_usable() {
    let factory = StdObserveFactory::create_factory();
    let tracer = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    let registry = factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry;
    let drain = factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain;

    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "pipeline_handler".to_string(),
            operation: "execute".to_string(),
        })
        .unwrap()
        .span;
    span.record(SpanAnnotationRequest {
        key: "db.rows".to_string(),
        value: "42".to_string(),
    })
    .unwrap();
    span.finish(SpanFinishRequest).unwrap();

    registry
        .counter(CounterLookupRequest {
            name: "pipeline.requests".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 1 })
        .unwrap();
    registry
        .histogram(HistogramLookupRequest {
            name: "pipeline.duration_ms".to_string(),
        })
        .unwrap()
        .histogram
        .record(HistogramRecordRequest { value: 25.0 })
        .unwrap();

    drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "pipeline_handler".to_string(),
            message: "done".to_string(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "all primitives work independently");
}

#[test]
fn test_observe_factory_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let factory = StdObserveFactory::create_factory();
    assert_send_sync(&factory);
    let tracer = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "factory is Send+Sync");
}
