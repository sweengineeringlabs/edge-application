use edge_application_base::SECURITY_CONTEXT_SVC_FACTORY;

#[test]
fn test_security_context_svc_factory_constant_value_happy() {
    assert_eq!(SECURITY_CONTEXT_SVC_FACTORY, "security_context_factory");
}

#[test]
fn test_security_context_svc_factory_constant_not_empty_error() {
    assert!(!SECURITY_CONTEXT_SVC_FACTORY.is_empty(), "SECURITY_CONTEXT_SVC_FACTORY must not be empty");
}

#[test]
fn test_security_context_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SECURITY_CONTEXT_SVC_FACTORY.contains(char::is_whitespace),
        "SECURITY_CONTEXT_SVC_FACTORY must not contain whitespace"
    );
}
