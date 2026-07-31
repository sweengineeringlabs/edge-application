//! Structural coverage for [`NameResponse`].

use edge_application_command::NameResponse;

/// @covers: NameResponse
#[test]
fn test_name_response_field_returns_configured_value_happy() {
    let response = NameResponse { name: "create-order".to_string() };
    assert_eq!(response.name, "create-order");
}

/// @covers: NameResponse
#[test]
fn test_name_response_default_is_empty_name_edge() {
    assert_eq!(NameResponse::default(), NameResponse { name: String::new() });
}

/// @covers: NameResponse
#[test]
fn test_name_response_clone_is_independent_edge() {
    let a = NameResponse { name: "a".to_string() };
    let b = a.clone();
    assert_eq!(a, b);
}
