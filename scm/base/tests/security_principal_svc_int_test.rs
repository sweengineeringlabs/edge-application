use edge_application_base::SECURITY_PRINCIPAL_SVC;

#[test]
fn test_security_principal_svc_constant_value_happy() {
    assert_eq!(SECURITY_PRINCIPAL_SVC, "security_principal");
}

#[test]
fn test_security_principal_svc_constant_not_empty_error() {
    assert!(!SECURITY_PRINCIPAL_SVC.is_empty(), "SECURITY_PRINCIPAL_SVC must not be empty");
}

#[test]
fn test_security_principal_svc_constant_no_whitespace_edge() {
    assert!(
        !SECURITY_PRINCIPAL_SVC.contains(char::is_whitespace),
        "SECURITY_PRINCIPAL_SVC must not contain whitespace"
    );
}
