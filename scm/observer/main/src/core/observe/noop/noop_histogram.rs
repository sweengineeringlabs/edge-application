use crate::api::Histogram;
use crate::api::HistogramRecordRequest;
use crate::api::HistogramRecordResponse;
use crate::api::NoopHistogram;
use crate::api::ObserveError;

impl Histogram for NoopHistogram {
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, ObserveError> {
        let _ = req;
        Ok(HistogramRecordResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_value_discarded_happy() {
        let h = NoopHistogram;
        h.record(HistogramRecordRequest { value: 25.0 }).unwrap();
        assert_eq!(std::mem::size_of_val(&h), 0);
    }

    #[test]
    fn test_record_zero_no_panic_error() {
        let h = NoopHistogram;
        h.record(HistogramRecordRequest { value: 0.0 }).unwrap();
        assert_eq!(std::mem::size_of_val(&h), 0);
    }

    #[test]
    fn test_noop_histogram_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopHistogram>(), 0);
    }
}
