//! Tests for the `PolicyError` error type.

use edge_application_policy::PolicyError;

/// @covers: PolicyError::new — carries policy name and reason
#[test]
fn test_new_carries_policy_and_reason_happy() {
    let v = PolicyError::new("limit", "exceeds 100");
    assert_eq!(v.policy, "limit");
    assert!(v.reason.contains("100"));
}

/// @covers: PolicyError — Display formats both fields
#[test]
fn test_display_includes_policy_and_reason_error() {
    let v = PolicyError::new("limit", "exceeds 100");
    let s = v.to_string();
    assert!(s.contains("limit"));
    assert!(s.contains("exceeds 100"));
}

/// @covers: PolicyError — equality by value
#[test]
fn test_equality_by_value_edge() {
    let a = PolicyError::new("p", "r");
    let b = PolicyError::new("p", "r");
    assert_eq!(a, b);
}
