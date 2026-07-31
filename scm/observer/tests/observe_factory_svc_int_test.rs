#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, HandlerTracerBuildRequest, IncrementRequest, LogDrainBuildRequest,
    LogEmitRequest, MetricRegistryBuildRequest, ObserveBootstrap, SpanFinishRequest,
    SpanStartRequest, StdObserveFactory, ValidationRequest, ValidationResponse,
    OBSERVE_FACTORY_SVC,
};

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_validate_passes_happy() {
    let factory = StdObserveFactory::create_factory();
    let result = factory.validate(ValidationRequest);
    assert_eq!(result, Ok(ValidationResponse), "created factory should validate successfully");
}

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_builds_noop_triplet_error() {
    let factory = StdObserveFactory::create_factory();
    let span = factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    factory
        .build_log_drain(LogDrainBuildRequest)
        .unwrap()
        .log_drain
        .emit(LogEmitRequest {
            level: "WARN".to_string(),
            handler_id: "h".to_string(),
            message: "bad input".to_string(),
        })
        .unwrap();
    factory
        .build_metric_registry(MetricRegistryBuildRequest)
        .unwrap()
        .metric_registry
        .counter(CounterLookupRequest {
            name: "errs".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 1 })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "factory builds ZST triplet");
}

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_called_repeatedly_edge() {
    let mut last_factory = StdObserveFactory::create_factory();
    for _ in 0..5 {
        last_factory = StdObserveFactory::create_factory();
    }
    let tracer = last_factory
        .build_handler_tracer(HandlerTracerBuildRequest)
        .unwrap()
        .tracer;
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "factories work when called repeatedly");
}

#[test]
fn test_observe_factory_svc_key_namespaced_happy() {
    assert!(OBSERVE_FACTORY_SVC.starts_with("edge."));
}
