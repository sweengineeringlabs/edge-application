//! Integration tests for `ResponseError`.

use edge_application_base::ResponseError;

/// @covers: ResponseError — is Debug-formattable (derives Debug)
#[test]
fn test_response_error_implements_debug_edge() {
    // ResponseError has no variants; verify the Debug impl compiles via a bound check.
    fn accepts_debug<T: std::fmt::Debug>(_: &T) {}
    let _ = accepts_debug::<ResponseError>; // verifies ResponseError: Debug at compile time

    assert!(!format!("{:?}", std::any::type_name::<ResponseError>()).is_empty());
}
