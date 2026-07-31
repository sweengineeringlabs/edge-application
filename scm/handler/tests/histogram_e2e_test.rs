//! SAF facade tests — `Histogram` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Mutex;

use edge_application_handler::{Histogram, HistogramRecordRequest, HistogramRecordResponse, ObserveError};

#[derive(Default)]
struct RecordingHistogram {
    observations: Mutex<Vec<f64>>,
}
impl Histogram for RecordingHistogram {
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, ObserveError> {
        self.observations.lock().unwrap().push(req.value);
        Ok(HistogramRecordResponse)
    }
}

/// @covers: Histogram::record — observation is stored
#[test]
fn test_record_single_observation_stored_happy() {
    let histogram = RecordingHistogram::default();
    histogram
        .record(HistogramRecordRequest { value: 120.0 })
        .expect("record should succeed");
    assert_eq!(*histogram.observations.lock().unwrap(), vec![120.0]);
}

/// @covers: Histogram::record — zero-value observation is still recorded
#[test]
fn test_record_zero_value_observation_stored_error() {
    let histogram = RecordingHistogram::default();
    histogram
        .record(HistogramRecordRequest { value: 0.0 })
        .expect("record should succeed");
    assert_eq!(*histogram.observations.lock().unwrap(), vec![0.0]);
}

/// @covers: Histogram::record — multiple observations accumulate in order
#[test]
fn test_record_multiple_observations_accumulate_in_order_edge() {
    let histogram = RecordingHistogram::default();
    histogram.record(HistogramRecordRequest { value: 1.0 }).unwrap();
    histogram.record(HistogramRecordRequest { value: 2.0 }).unwrap();
    assert_eq!(*histogram.observations.lock().unwrap(), vec![1.0, 2.0]);
}
