//! Integration tests for `RequestError`.

use edge_application_base::RequestError;

/// @covers: RequestError — is Debug-formattable (derives Debug)
#[test]
fn test_request_error_implements_debug_edge() {
    // RequestError has no variants; verify the Debug impl compiles via a bound check.
    fn accepts_debug<T: std::fmt::Debug>(_: &T) {}
    let _ = accepts_debug::<RequestError>; // verifies RequestError: Debug at compile time

    assert!(!format!("{:?}", std::any::type_name::<RequestError>()).is_empty());
}
