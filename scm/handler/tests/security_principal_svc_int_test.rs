//! Integration tests — `SECURITY_PRINCIPAL_SVC` constant.

use edge_application_handler::SECURITY_PRINCIPAL_SVC;

/// @covers: SECURITY_PRINCIPAL_SVC — correct service identity value
#[test]
fn test_security_principal_svc_constant_value_happy() {
    assert_eq!(SECURITY_PRINCIPAL_SVC, "security_principal");
}

/// @covers: SECURITY_PRINCIPAL_SVC — constant is non-empty
#[test]
fn test_security_principal_svc_constant_not_empty_error() {
    assert!(!SECURITY_PRINCIPAL_SVC.is_empty());
    assert_eq!(SECURITY_PRINCIPAL_SVC.len(), "security_principal".len());
}

/// @covers: SECURITY_PRINCIPAL_SVC — constant contains no whitespace
#[test]
fn test_security_principal_svc_constant_no_whitespace_edge() {
    assert!(!SECURITY_PRINCIPAL_SVC.contains(' '));
    assert!(!SECURITY_PRINCIPAL_SVC.contains('\t'));
    assert_eq!(SECURITY_PRINCIPAL_SVC, SECURITY_PRINCIPAL_SVC.trim());
}
