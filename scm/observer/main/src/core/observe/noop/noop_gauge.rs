use crate::api::Gauge;
use crate::api::GaugeSetRequest;
use crate::api::GaugeSetResponse;
use crate::api::NoopGauge;
use crate::api::ObserveError;

impl Gauge for NoopGauge {
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, ObserveError> {
        let _ = req;
        Ok(GaugeSetResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_value_discarded_happy() {
        let g = NoopGauge;
        g.set(GaugeSetRequest { value: 42.0 }).unwrap();
        assert_eq!(std::mem::size_of_val(&g), 0);
    }

    #[test]
    fn test_set_negative_value_no_panic_error() {
        let g = NoopGauge;
        g.set(GaugeSetRequest { value: -1.0 }).unwrap();
        assert_eq!(std::mem::size_of_val(&g), 0);
    }

    #[test]
    fn test_noop_gauge_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopGauge>(), 0);
    }
}
