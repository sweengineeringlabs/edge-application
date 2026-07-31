//! Integration tests for `ValueObjectError`.

use edge_application_valueobject::ValueObjectError;

/// @covers: ValueObjectError::Empty — Display message
#[test]
fn test_value_object_error_empty_displays_correct_message_happy() {
    let e = ValueObjectError::Empty;
    assert_eq!(e.to_string(), "value must not be empty");
}

/// @covers: ValueObjectError::Invalid — Display includes the message
#[test]
fn test_value_object_error_invalid_displays_message_happy() {
    let e = ValueObjectError::Invalid("bad format".to_string());
    assert!(e.to_string().contains("bad format"));
}

/// @covers: ValueObjectError — implements Debug
#[test]
fn test_value_object_error_implements_debug_edge() {
    let e = ValueObjectError::Empty;
    let debug_str = format!("{e:?}");
    assert!(!debug_str.is_empty());
}
