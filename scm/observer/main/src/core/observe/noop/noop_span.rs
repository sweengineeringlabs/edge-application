use crate::api::NoopSpan;
use crate::api::ObserveError;
use crate::api::Span;
use crate::api::SpanAnnotationRequest;
use crate::api::SpanAnnotationResponse;
use crate::api::SpanFinishRequest;
use crate::api::SpanFinishResponse;

impl Span for NoopSpan {
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, ObserveError> {
        let _ = req;
        Ok(SpanAnnotationResponse)
    }

    fn finish(&self, req: SpanFinishRequest) -> Result<SpanFinishResponse, ObserveError> {
        let _ = (self, req);
        Ok(SpanFinishResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_key_value_discarded_happy() {
        let s = NoopSpan;
        s.record(SpanAnnotationRequest {
            key: "k".to_string(),
            value: "v".to_string(),
        })
        .unwrap();
        assert_eq!(std::mem::size_of_val(&s), 0);
    }

    #[test]
    fn test_finish_completes_without_panic_error() {
        let s = NoopSpan;
        s.finish(SpanFinishRequest).unwrap();
        assert_eq!(std::mem::size_of_val(&s), 0);
    }

    #[test]
    fn test_noop_span_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopSpan>(), 0);
    }
}
