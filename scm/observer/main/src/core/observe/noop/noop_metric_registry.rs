use crate::api::Counter;
use crate::api::CounterLookupRequest;
use crate::api::CounterLookupResponse;
use crate::api::Gauge;
use crate::api::GaugeLookupRequest;
use crate::api::GaugeLookupResponse;
use crate::api::Histogram;
use crate::api::HistogramLookupRequest;
use crate::api::HistogramLookupResponse;
use crate::api::MetricRegistry;
use crate::api::NoopCounter;
use crate::api::NoopGauge;
use crate::api::NoopHistogram;
use crate::api::NoopMetricRegistry;
use crate::api::ObserveError;

impl NoopMetricRegistry {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl MetricRegistry for NoopMetricRegistry {
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, ObserveError> {
        let _ = req;
        Ok(CounterLookupResponse {
            counter: Box::new(NoopCounter) as Box<dyn Counter>,
        })
    }

    fn histogram(
        &self,
        req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, ObserveError> {
        let _ = req;
        Ok(HistogramLookupResponse {
            histogram: Box::new(NoopHistogram) as Box<dyn Histogram>,
        })
    }

    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, ObserveError> {
        let _ = req;
        Ok(GaugeLookupResponse {
            gauge: Box::new(NoopGauge) as Box<dyn Gauge>,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{GaugeSetRequest, HistogramRecordRequest, IncrementRequest};

    fn name(name: &str) -> String {
        name.to_string()
    }

    #[test]
    fn test_new_creates_noop_metric_registry_happy() {
        let r = NoopMetricRegistry::new();
        assert_eq!(std::mem::size_of_val(&r), 0);
    }

    #[test]
    fn test_counter_returns_usable_counter_error() {
        let r = NoopMetricRegistry::new();
        r.counter(CounterLookupRequest { name: name("errs") })
            .unwrap()
            .counter
            .increment(IncrementRequest { delta: u64::MAX })
            .unwrap();
        assert_eq!(std::mem::size_of_val(&r), 0);
    }

    #[test]
    fn test_all_three_primitives_constructible_edge() {
        let r = NoopMetricRegistry::new();
        r.counter(CounterLookupRequest { name: name("c") })
            .unwrap()
            .counter
            .increment(IncrementRequest { delta: 1 })
            .unwrap();
        r.histogram(HistogramLookupRequest { name: name("h") })
            .unwrap()
            .histogram
            .record(HistogramRecordRequest { value: 1.0 })
            .unwrap();
        r.gauge(GaugeLookupRequest { name: name("g") })
            .unwrap()
            .gauge
            .set(GaugeSetRequest { value: 1.0 })
            .unwrap();
        assert_eq!(std::mem::size_of_val(&r), 0);
    }
}
