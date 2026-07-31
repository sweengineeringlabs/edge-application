#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    HandlerTracer, SpanAnnotationRequest, SpanFinishRequest, SpanStartRequest, StdObserveFactory,
};

// --- start_span (HandlerTracer) ---

#[test]
fn test_start_span_handler_and_op_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "handler_b".to_string(),
            operation: "validate".to_string(),
        })
        .unwrap()
        .span;
    span.record(SpanAnnotationRequest {
        key: "result".to_string(),
        value: "ok".to_string(),
    })
    .unwrap();
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "noop span is a ZST");
}

#[test]
fn test_start_span_empty_handler_id_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "noop span handles empty ids");
}

#[test]
fn test_start_span_very_long_ids_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let long_id = "x".repeat(1024);
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: long_id.clone(),
            operation: long_id,
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "noop span handles long ids");
}

#[test]
fn test_handler_tracer_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let tracer = StdObserveFactory::noop_handler_tracer();
    assert_send_sync(&tracer);
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "noop handler tracer is ZST");
}

#[test]
fn test_handler_tracer_returns_dyn_trait_object() {
    let tracer: Box<dyn HandlerTracer> = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "noop span is ZST");
}
