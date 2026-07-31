//! Tests for the `ClockError` error type.

use edge_application_clock::ClockError;

/// @covers: ClockError::BeforeEpoch — Display
#[test]
fn test_before_epoch_display_mentions_epoch_happy() {
    assert!(ClockError::BeforeEpoch.to_string().contains("epoch"));
}

/// @covers: ClockError — equality
#[test]
fn test_before_epoch_equals_itself_error() {
    let err1 = ClockError::BeforeEpoch;
    let err2 = ClockError::BeforeEpoch;
    assert_eq!(err1, err2, "same variant must be equal");
}

/// @covers: ClockError — Debug formatting
#[test]
fn test_before_epoch_debug_is_variant_name_edge() {
    assert_eq!(format!("{:?}", ClockError::BeforeEpoch), "BeforeEpoch");
}
