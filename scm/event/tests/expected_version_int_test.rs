//! Integration tests for `ExpectedVersion`.

use edge_application_event::ExpectedVersion;

/// @covers: ExpectedVersion — Any variant creation and identity
#[test]
fn test_expected_version_any_equals_any_happy() {
    let v1 = ExpectedVersion::Any;
    let v2 = ExpectedVersion::Any;
    // Verify that Any variants are equal to each other (not just self-comparison)
    assert_eq!(v1, v2);
}

/// @covers: ExpectedVersion — NoStream not equal to Any
#[test]
fn test_expected_version_no_stream_not_equal_any_error() {
    assert_ne!(ExpectedVersion::NoStream, ExpectedVersion::Any);
}

/// @covers: ExpectedVersion — Exact wraps and exposes value
#[test]
fn test_expected_version_exact_wraps_value_edge() {
    let v = ExpectedVersion::Exact(42);
    assert_eq!(v, ExpectedVersion::Exact(42));
    assert_ne!(v, ExpectedVersion::Exact(0));
}
