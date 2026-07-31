//! Structural coverage for [`NameRequest`].

use edge_application_command::NameRequest;

/// @covers: NameRequest
#[test]
fn test_name_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NameRequest>(), 0);
}

/// @covers: NameRequest
#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_name_request_default_equals_unit_value_happy() {
    assert_eq!(NameRequest::default(), NameRequest);
}

/// @covers: NameRequest
#[test]
fn test_name_request_is_copy_edge() {
    let a = NameRequest;
    let b = a;
    assert_eq!(a, b);
}
