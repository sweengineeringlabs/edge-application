use crate::api::DrainRequest;
use crate::api::DrainResponse;
use crate::api::HandlerTracer;
use crate::api::LogDrain;
use crate::api::MetricRegistry;
use crate::api::MetricsRequest;
use crate::api::MetricsResponse;
use crate::api::NoopHandlerTracer;
use crate::api::NoopLogDrain;
use crate::api::NoopMetricRegistry;
use crate::api::NoopObserverContext;
use crate::api::ObserveError;
use crate::api::ObserverContext;
use crate::api::TracerRequest;
use crate::api::TracerResponse;

impl NoopObserverContext {
    pub(crate) fn new() -> Self {
        Self {
            tracer: Box::new(NoopHandlerTracer::new()),
            drain: Box::new(NoopLogDrain::new()),
            metrics: Box::new(NoopMetricRegistry::new()),
        }
    }
}

impl ObserverContext for NoopObserverContext {
    fn tracer(&self, _req: TracerRequest) -> Result<TracerResponse<'_>, ObserveError> {
        Ok(TracerResponse {
            tracer: self.tracer.as_ref() as &dyn HandlerTracer,
        })
    }

    fn drain(&self, _req: DrainRequest) -> Result<DrainResponse<'_>, ObserveError> {
        Ok(DrainResponse {
            drain: self.drain.as_ref() as &dyn LogDrain,
        })
    }

    fn metrics(&self, _req: MetricsRequest) -> Result<MetricsResponse<'_>, ObserveError> {
        Ok(MetricsResponse {
            metrics: self.metrics.as_ref() as &dyn MetricRegistry,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{CounterLookupRequest, IncrementRequest, SpanFinishRequest, SpanStartRequest};

    #[test]
    fn test_new_creates_noop_observer_context_happy() {
        let ctx = NoopObserverContext::new();
        let span = ctx
            .tracer(TracerRequest)
            .unwrap()
            .tracer
            .start_span(SpanStartRequest {
                handler_id: "h".to_string(),
                operation: "op".to_string(),
            })
            .unwrap()
            .span;
        span.finish(SpanFinishRequest).unwrap();
        assert_eq!(
            std::mem::size_of_val(&*span),
            0,
            "noop observer context is a ZST"
        );
    }

    #[test]
    fn test_tracer_start_span_no_panic_error() {
        let ctx = NoopObserverContext::new();
        let span = ctx
            .tracer(TracerRequest)
            .unwrap()
            .tracer
            .start_span(SpanStartRequest {
                handler_id: "h".to_string(),
                operation: "op".to_string(),
            })
            .unwrap()
            .span;
        span.finish(SpanFinishRequest).unwrap();
        assert_eq!(std::mem::size_of_val(&*span), 0, "tracer span is a ZST");
    }

    #[test]
    fn test_metrics_counter_no_panic_edge() {
        let ctx = NoopObserverContext::new();
        let counter = ctx
            .metrics(MetricsRequest)
            .unwrap()
            .metrics
            .counter(CounterLookupRequest {
                name: "m".to_string(),
            })
            .unwrap()
            .counter;
        counter.increment(IncrementRequest { delta: 1 }).unwrap();
        assert_eq!(
            std::mem::size_of_val(&*counter),
            0,
            "metric counter is a ZST"
        );
    }
}
