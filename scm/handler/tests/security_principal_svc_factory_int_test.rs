//! Integration tests — `SECURITY_PRINCIPAL_SVC_FACTORY` constant.

use edge_application_handler::SECURITY_PRINCIPAL_SVC_FACTORY;

/// @covers: SECURITY_PRINCIPAL_SVC_FACTORY — correct factory identity value
#[test]
fn test_security_principal_svc_factory_constant_value_happy() {
    assert_eq!(SECURITY_PRINCIPAL_SVC_FACTORY, "security_principal_factory");
}

/// @covers: SECURITY_PRINCIPAL_SVC_FACTORY — constant is non-empty
#[test]
fn test_security_principal_svc_factory_constant_not_empty_error() {
    assert!(!SECURITY_PRINCIPAL_SVC_FACTORY.is_empty());
    assert_eq!(
        SECURITY_PRINCIPAL_SVC_FACTORY.len(),
        "security_principal_factory".len()
    );
}

/// @covers: SECURITY_PRINCIPAL_SVC_FACTORY — constant contains no whitespace
#[test]
fn test_security_principal_svc_factory_constant_no_whitespace_edge() {
    assert!(!SECURITY_PRINCIPAL_SVC_FACTORY.contains(' '));
    assert!(!SECURITY_PRINCIPAL_SVC_FACTORY.contains('\t'));
    assert_eq!(
        SECURITY_PRINCIPAL_SVC_FACTORY,
        SECURITY_PRINCIPAL_SVC_FACTORY.trim()
    );
}
