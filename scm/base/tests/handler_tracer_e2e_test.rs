//! SAF facade tests — `HandlerTracer` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Mutex;

use edge_application_base::{
    HandlerTracer, ObserveError, Span, SpanAnnotationRequest, SpanAnnotationResponse,
    SpanFinishRequest, SpanFinishResponse, SpanStartRequest, SpanStartResponse,
};

struct NamedSpan;
impl Span for NamedSpan {
    fn record(&self, _req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, ObserveError> {
        Ok(SpanAnnotationResponse)
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, ObserveError> {
        Ok(SpanFinishResponse)
    }
}

/// Records the `(handler_id, operation)` pair passed to every `start_span` call so
/// the test can assert the tracer actually forwarded the request, not just that it
/// returned *a* span.
#[derive(Default)]
struct RecordingTracer {
    calls: Mutex<Vec<(String, String)>>,
}
impl HandlerTracer for RecordingTracer {
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, ObserveError> {
        self.calls
            .lock()
            .unwrap()
            .push((req.handler_id.clone(), req.operation.clone()));
        Ok(SpanStartResponse {
            span: Box::new(NamedSpan),
        })
    }
}

/// @covers: HandlerTracer::start_span — forwards handler_id/operation and returns a usable span
#[test]
fn test_start_span_forwards_request_fields_happy() {
    let tracer = RecordingTracer::default();
    let response = tracer
        .start_span(SpanStartRequest {
            handler_id: "order".into(),
            operation: "create".into(),
        })
        .expect("start_span should succeed");
    assert_eq!(
        *tracer.calls.lock().unwrap(),
        vec![("order".to_string(), "create".to_string())]
    );
    assert_eq!(
        response.span.finish(SpanFinishRequest),
        Ok(SpanFinishResponse)
    );
}

/// @covers: HandlerTracer::start_span — empty operation name is still forwarded, not rejected
#[test]
fn test_start_span_empty_operation_still_succeeds_error() {
    let tracer = RecordingTracer::default();
    tracer
        .start_span(SpanStartRequest {
            handler_id: "order".into(),
            operation: String::new(),
        })
        .expect("start_span should succeed");
    assert_eq!(
        *tracer.calls.lock().unwrap(),
        vec![("order".to_string(), String::new())]
    );
}

/// @covers: HandlerTracer::start_span — repeated calls each return an independent span
#[test]
fn test_start_span_repeated_calls_return_independent_spans_edge() {
    let tracer = RecordingTracer::default();
    let first = tracer
        .start_span(SpanStartRequest {
            handler_id: "a".into(),
            operation: "op".into(),
        })
        .expect("start_span should succeed");
    let second = tracer
        .start_span(SpanStartRequest {
            handler_id: "b".into(),
            operation: "op".into(),
        })
        .expect("start_span should succeed");
    assert!(first.span.finish(SpanFinishRequest).is_ok());
    assert!(second.span.finish(SpanFinishRequest).is_ok());
    assert_eq!(tracer.calls.lock().unwrap().len(), 2);
}
