//! SAF facade tests — `Span` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Mutex;

use edge_application_base::{
    ObserveError, Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse,
};

#[derive(Default)]
struct RecordingSpan {
    annotations: Mutex<Vec<(String, String)>>,
    finish_count: Mutex<u32>,
}
impl Span for RecordingSpan {
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, ObserveError> {
        self.annotations.lock().unwrap().push((req.key, req.value));
        Ok(SpanAnnotationResponse)
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, ObserveError> {
        *self.finish_count.lock().unwrap() += 1;
        Ok(SpanFinishResponse)
    }
}

/// @covers: Span::record — annotation key/value is stored
#[test]
fn test_record_annotation_stores_key_value_happy() {
    let span = RecordingSpan::default();
    span.record(SpanAnnotationRequest {
        key: "order_id".into(),
        value: "42".into(),
    })
    .expect("record should succeed");
    assert_eq!(
        span.annotations.lock().unwrap()[0],
        ("order_id".to_string(), "42".to_string())
    );
}

/// @covers: Span::record — empty value is still recorded, not rejected
#[test]
fn test_record_empty_value_still_stored_error() {
    let span = RecordingSpan::default();
    span.record(SpanAnnotationRequest {
        key: "note".into(),
        value: String::new(),
    })
    .expect("record should succeed");
    assert_eq!(span.annotations.lock().unwrap()[0].1, "");
}

/// @covers: Span::record — repeated annotations accumulate in order
#[test]
fn test_record_repeated_annotations_accumulate_in_order_edge() {
    let span = RecordingSpan::default();
    span.record(SpanAnnotationRequest {
        key: "a".into(),
        value: "1".into(),
    })
    .unwrap();
    span.record(SpanAnnotationRequest {
        key: "b".into(),
        value: "2".into(),
    })
    .unwrap();
    let annotations = span.annotations.lock().unwrap();
    assert_eq!(annotations.len(), 2);
    assert_eq!(annotations[1], ("b".to_string(), "2".to_string()));
}

/// @covers: Span::finish — marks span finished
#[test]
fn test_finish_called_once_marks_span_finished_happy() {
    let span = RecordingSpan::default();
    span.finish(SpanFinishRequest).expect("finish should succeed");
    assert_eq!(*span.finish_count.lock().unwrap(), 1);
}

/// @covers: Span::finish — finishing an already-annotated span still succeeds
#[test]
fn test_finish_after_annotations_still_succeeds_error() {
    let span = RecordingSpan::default();
    span.record(SpanAnnotationRequest {
        key: "k".into(),
        value: "v".into(),
    })
    .unwrap();
    assert!(span.finish(SpanFinishRequest).is_ok());
}

/// @covers: Span::finish — calling finish more than once is observable, not silently ignored
#[test]
fn test_finish_called_twice_increments_count_edge() {
    let span = RecordingSpan::default();
    span.finish(SpanFinishRequest).unwrap();
    span.finish(SpanFinishRequest).unwrap();
    assert_eq!(*span.finish_count.lock().unwrap(), 2);
}
