//! Tests for the `QueryError` error type.

use edge_application_query::QueryError;

/// @covers: QueryError::InvalidInput — Display
#[test]
fn test_invalid_input_display_includes_message_happy() {
    let e = QueryError::InvalidInput("bad id".into());
    assert!(e.to_string().contains("bad id"));
}

/// @covers: QueryError::NotFound — Display
#[test]
fn test_not_found_display_includes_key_error() {
    let e = QueryError::NotFound("order-99".into());
    assert!(e.to_string().contains("order-99"));
}

/// @covers: QueryError — variants format distinctly
#[test]
fn test_variants_format_distinctly_edge() {
    let nf = QueryError::NotFound("x".into()).to_string();
    let internal = QueryError::Internal("y".into()).to_string();
    assert!(nf.contains("not found"));
    assert!(internal.contains("internal"));
    assert_ne!(nf, internal);
}
