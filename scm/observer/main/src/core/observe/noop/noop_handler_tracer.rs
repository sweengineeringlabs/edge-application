use crate::api::HandlerTracer;
use crate::api::NoopHandlerTracer;
use crate::api::NoopSpan;
use crate::api::ObserveError;
use crate::api::Span;
use crate::api::SpanStartRequest;
use crate::api::SpanStartResponse;

impl NoopHandlerTracer {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl HandlerTracer for NoopHandlerTracer {
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, ObserveError> {
        let _ = req;
        Ok(SpanStartResponse {
            span: Box::new(NoopSpan) as Box<dyn Span>,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::SpanFinishRequest;

    fn start(handler_id: &str, operation: &str) -> SpanStartRequest {
        SpanStartRequest {
            handler_id: handler_id.to_string(),
            operation: operation.to_string(),
        }
    }

    #[test]
    fn test_new_creates_noop_handler_tracer_happy() {
        let t = NoopHandlerTracer::new();
        assert_eq!(std::mem::size_of_val(&t), 0);
    }

    #[test]
    fn test_start_span_empty_ids_no_panic_error() {
        let t = NoopHandlerTracer::new();
        let span = t.start_span(start("", "")).unwrap().span;
        span.finish(SpanFinishRequest).unwrap();
        assert_eq!(std::mem::size_of_val(&t), 0);
    }

    #[test]
    fn test_start_span_multiple_calls_independent_edge() {
        let t = NoopHandlerTracer::new();
        t.start_span(start("a", "op1"))
            .unwrap()
            .span
            .finish(SpanFinishRequest)
            .unwrap();
        t.start_span(start("b", "op2"))
            .unwrap()
            .span
            .finish(SpanFinishRequest)
            .unwrap();
        assert_eq!(std::mem::size_of_val(&t), 0);
    }
}
