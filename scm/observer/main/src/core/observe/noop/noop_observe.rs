use crate::api::Counter;
use crate::api::Gauge;
use crate::api::HandlerTracer;
use crate::api::Histogram;
use crate::api::LogDrain;
use crate::api::MetricRegistry;
use crate::api::NoopCounter;
use crate::api::NoopGauge;
use crate::api::NoopHandlerTracer;
use crate::api::NoopHistogram;
use crate::api::NoopLogDrain;
use crate::api::NoopMetricRegistry;
use crate::api::NoopObserve;
use crate::api::NoopObserverContext;
use crate::api::NoopSpan;
use crate::api::ObserverContext;
use crate::api::Span;
use crate::api::StdObserveFactory;

impl NoopObserve for StdObserveFactory {
    fn build_noop_counter() -> Box<dyn Counter> {
        Box::new(NoopCounter)
    }

    fn build_noop_gauge() -> Box<dyn Gauge> {
        Box::new(NoopGauge)
    }

    fn build_noop_histogram() -> Box<dyn Histogram> {
        Box::new(NoopHistogram)
    }

    fn build_noop_span() -> Box<dyn Span> {
        Box::new(NoopSpan)
    }

    fn build_noop_handler_tracer() -> Box<dyn HandlerTracer> {
        Box::new(NoopHandlerTracer::new())
    }

    fn build_noop_log_drain() -> Box<dyn LogDrain> {
        Box::new(NoopLogDrain::new())
    }

    fn build_noop_metric_registry() -> Box<dyn MetricRegistry> {
        Box::new(NoopMetricRegistry::new())
    }

    fn build_noop_observer_context() -> Box<dyn ObserverContext> {
        Box::new(NoopObserverContext::new())
    }
}
