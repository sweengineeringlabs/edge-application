//! Tests for the `SnapshotError` error type.

use edge_application_snapshot::SnapshotError;

/// @covers: SnapshotError::InvalidVersion — Display
#[test]
fn test_invalid_version_display_includes_id_and_version_happy() {
    let e = SnapshotError::InvalidVersion {
        aggregate_id: "order-1".into(),
        version: 0,
    };
    let s = e.to_string();
    assert!(s.contains("order-1"));
    assert!(s.contains('0'));
}

/// @covers: SnapshotError::Unavailable — Display
#[test]
fn test_unavailable_display_includes_reason_error() {
    let e = SnapshotError::Unavailable("timeout".into());
    assert!(e.to_string().contains("timeout"));
}

/// @covers: SnapshotError — variants format distinctly
#[test]
fn test_variants_format_distinctly_edge() {
    let inv = SnapshotError::InvalidVersion {
        aggregate_id: "x".into(),
        version: 0,
    }
    .to_string();
    let int = SnapshotError::Internal("y".into()).to_string();
    assert_ne!(inv, int);
}
