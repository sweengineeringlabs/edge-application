#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    SpanAnnotationRequest, SpanFinishRequest, SpanStartRequest, StdObserveFactory,
};

// --- start_span ---

#[test]
fn test_start_span_with_valid_ids_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "handler_a".to_string(),
            operation: "execute".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    // Span was created and finished without error.
    assert_eq!(std::mem::size_of_val(&*span), 0);
}

#[test]
fn test_start_span_empty_ids_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "".to_string(),
            operation: "".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    // Empty IDs are handled without error.
    assert_eq!(std::mem::size_of_val(&*span), 0);
}

#[test]
fn test_start_span_unicode_ids_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "handler_α".to_string(),
            operation: "执行".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "unicode ids work");
}

// --- record (Span) ---

#[test]
fn test_record_key_value_pair_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.record(SpanAnnotationRequest {
        key: "http.method".to_string(),
        value: "GET".to_string(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "record works");
}

#[test]
fn test_record_empty_key_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.record(SpanAnnotationRequest {
        key: "".to_string(),
        value: "value".to_string(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "empty key handled");
}

#[test]
fn test_record_unicode_value_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.record(SpanAnnotationRequest {
        key: "key".to_string(),
        value: "值".to_string(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "unicode value works");
}

// --- finish ---

#[test]
fn test_finish_active_span_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "finish works");
}

#[test]
fn test_finish_already_done_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "double finish handled");
}

#[test]
fn test_finish_span_no_records_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "finish without records works");
}

#[test]
fn test_span_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span;
    assert_send_sync(&span);
    assert_eq!(std::mem::size_of_val(&*span), 0, "span is Send+Sync ZST");
}
