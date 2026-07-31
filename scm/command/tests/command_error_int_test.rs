//! Tests for the `CommandError` error type.

use edge_application_command::CommandError;

/// @covers: CommandError::InvalidInput — Display
#[test]
fn test_invalid_input_display_includes_message_happy() {
    let e = CommandError::InvalidInput("bad id".into());
    assert!(e.to_string().contains("bad id"));
}

/// @covers: CommandError::RuleViolation — Display
#[test]
fn test_rule_violation_display_includes_message_error() {
    let e = CommandError::RuleViolation("over limit".into());
    assert!(e.to_string().contains("over limit"));
}

/// @covers: CommandError — each variant is distinct
#[test]
fn test_variants_format_distinctly_edge() {
    let nf = CommandError::NotFound("x".into()).to_string();
    let internal = CommandError::Internal("y".into()).to_string();
    assert!(nf.contains("not found"));
    assert!(internal.contains("internal"));
    assert_ne!(nf, internal);
}
