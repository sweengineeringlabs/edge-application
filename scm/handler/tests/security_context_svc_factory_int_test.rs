//! Integration tests — `SECURITY_CONTEXT_SVC_FACTORY` constant.

use edge_application_handler::SECURITY_CONTEXT_SVC_FACTORY;

/// @covers: SECURITY_CONTEXT_SVC_FACTORY — correct factory identity value
#[test]
fn test_security_context_svc_factory_constant_value_happy() {
    assert_eq!(SECURITY_CONTEXT_SVC_FACTORY, "security_context_factory");
}

/// @covers: SECURITY_CONTEXT_SVC_FACTORY — constant is non-empty
#[test]
fn test_security_context_svc_factory_constant_not_empty_error() {
    assert!(!SECURITY_CONTEXT_SVC_FACTORY.is_empty());
    assert_eq!(
        SECURITY_CONTEXT_SVC_FACTORY.len(),
        "security_context_factory".len()
    );
}

/// @covers: SECURITY_CONTEXT_SVC_FACTORY — constant contains no whitespace
#[test]
fn test_security_context_svc_factory_constant_no_whitespace_edge() {
    assert!(!SECURITY_CONTEXT_SVC_FACTORY.contains(' '));
    assert!(!SECURITY_CONTEXT_SVC_FACTORY.contains('\t'));
    assert_eq!(
        SECURITY_CONTEXT_SVC_FACTORY,
        SECURITY_CONTEXT_SVC_FACTORY.trim()
    );
}
