use edge_application_base::SECURITY_PRINCIPAL_SVC_FACTORY;

#[test]
fn test_security_principal_svc_factory_constant_value_happy() {
    assert_eq!(SECURITY_PRINCIPAL_SVC_FACTORY, "security_principal_factory");
}

#[test]
fn test_security_principal_svc_factory_constant_not_empty_error() {
    assert!(!SECURITY_PRINCIPAL_SVC_FACTORY.is_empty(), "SECURITY_PRINCIPAL_SVC_FACTORY must not be empty");
}

#[test]
fn test_security_principal_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SECURITY_PRINCIPAL_SVC_FACTORY.contains(char::is_whitespace),
        "SECURITY_PRINCIPAL_SVC_FACTORY must not contain whitespace"
    );
}
