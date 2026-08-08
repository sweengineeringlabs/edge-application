//! Integration tests — `SECURITY_CONTEXT_SVC` constant.

use edge_application_handler::SECURITY_CONTEXT_SVC;

/// @covers: SECURITY_CONTEXT_SVC — correct service identity value
#[test]
fn test_security_context_svc_constant_value_happy() {
    assert_eq!(SECURITY_CONTEXT_SVC, "security_context");
}

/// @covers: SECURITY_CONTEXT_SVC — constant is non-empty
#[test]
fn test_security_context_svc_constant_not_empty_error() {
    assert!(!SECURITY_CONTEXT_SVC.is_empty());
    assert_eq!(SECURITY_CONTEXT_SVC.len(), "security_context".len());
}

/// @covers: SECURITY_CONTEXT_SVC — constant contains no whitespace
#[test]
fn test_security_context_svc_constant_no_whitespace_edge() {
    assert!(!SECURITY_CONTEXT_SVC.contains(' '));
    assert!(!SECURITY_CONTEXT_SVC.contains('\t'));
    assert_eq!(SECURITY_CONTEXT_SVC, SECURITY_CONTEXT_SVC.trim());
}
