//! SAF facade tests — `Gauge` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::atomic::{AtomicU64, Ordering};

use edge_application_handler::{Gauge, GaugeSetRequest, GaugeSetResponse, ObserveError};

#[derive(Default)]
struct RecordingGauge {
    last_bits: AtomicU64,
}
impl Gauge for RecordingGauge {
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, ObserveError> {
        self.last_bits.store(req.value.to_bits(), Ordering::SeqCst);
        Ok(GaugeSetResponse)
    }
}

/// @covers: Gauge::set — recorded value is applied
#[test]
fn test_set_positive_value_updates_gauge_happy() {
    let gauge = RecordingGauge::default();
    gauge
        .set(GaugeSetRequest { value: 42.5 })
        .expect("set should succeed");
    assert_eq!(f64::from_bits(gauge.last_bits.load(Ordering::SeqCst)), 42.5);
}

/// @covers: Gauge::set — negative value is stored as-is
#[test]
fn test_set_negative_value_updates_gauge_error() {
    let gauge = RecordingGauge::default();
    gauge
        .set(GaugeSetRequest { value: -3.0 })
        .expect("set should succeed");
    assert_eq!(f64::from_bits(gauge.last_bits.load(Ordering::SeqCst)), -3.0);
}

/// @covers: Gauge::set — later call overwrites earlier one
#[test]
fn test_set_repeated_calls_overwrite_previous_edge() {
    let gauge = RecordingGauge::default();
    gauge.set(GaugeSetRequest { value: 1.0 }).unwrap();
    gauge.set(GaugeSetRequest { value: 2.0 }).unwrap();
    assert_eq!(f64::from_bits(gauge.last_bits.load(Ordering::SeqCst)), 2.0);
}
