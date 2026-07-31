//! Structural coverage for [`ExecutionRequest`].

use edge_application_command::ExecutionRequest;

/// @covers: ExecutionRequest
#[test]
fn test_execution_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<ExecutionRequest>(), 0);
}

/// @covers: ExecutionRequest
#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_execution_request_default_equals_unit_value_happy() {
    assert_eq!(ExecutionRequest::default(), ExecutionRequest);
}

/// @covers: ExecutionRequest
#[test]
fn test_execution_request_is_copy_edge() {
    let a = ExecutionRequest;
    let b = a;
    assert_eq!(a, b);
}
