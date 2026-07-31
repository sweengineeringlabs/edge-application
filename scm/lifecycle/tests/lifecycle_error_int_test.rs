//! Integration tests for `LifecycleError` — covers the errors/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::LifecycleError;

/// @covers: LifecycleError::InvalidTransition — display message includes from and to
#[test]
fn test_invalid_transition_display_includes_from_and_to_happy() {
    let e = LifecycleError::InvalidTransition {
        from: "Idle".to_string(),
        to: "Stopped".to_string(),
    };
    let msg = e.to_string();
    assert!(msg.contains("Idle"), "expected 'Idle' in: {msg}");
    assert!(msg.contains("Stopped"), "expected 'Stopped' in: {msg}");
    assert!(msg.contains("not allowed"), "expected 'not allowed' in: {msg}");
}

/// @covers: LifecycleError::InvalidTransition — equality discriminates on both fields
#[test]
fn test_invalid_transition_equality_discriminates_both_fields_error() {
    let e1 = LifecycleError::InvalidTransition {
        from: "A".to_string(),
        to: "B".to_string(),
    };
    let e2 = LifecycleError::InvalidTransition {
        from: "A".to_string(),
        to: "B".to_string(),
    };
    let e3 = LifecycleError::InvalidTransition {
        from: "A".to_string(),
        to: "C".to_string(),
    };
    assert_eq!(e1, e2);
    assert_ne!(e1, e3);
}

/// @covers: LifecycleError::InvalidTransition — empty from/to still renders
#[test]
fn test_invalid_transition_empty_fields_renders_message_edge() {
    let e = LifecycleError::InvalidTransition {
        from: String::new(),
        to: String::new(),
    };
    assert!(e.to_string().contains("not allowed"));
}
