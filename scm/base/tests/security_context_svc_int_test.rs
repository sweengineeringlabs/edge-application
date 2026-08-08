use edge_application_base::SECURITY_CONTEXT_SVC;

#[test]
fn test_security_context_svc_constant_value_happy() {
    assert_eq!(SECURITY_CONTEXT_SVC, "security_context");
}

#[test]
fn test_security_context_svc_constant_not_empty_error() {
    assert!(!SECURITY_CONTEXT_SVC.is_empty(), "SECURITY_CONTEXT_SVC must not be empty");
}

#[test]
fn test_security_context_svc_constant_no_whitespace_edge() {
    assert!(
        !SECURITY_CONTEXT_SVC.contains(char::is_whitespace),
        "SECURITY_CONTEXT_SVC must not contain whitespace"
    );
}
