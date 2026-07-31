use edge_application_repository::{SPEC_SVC, SPEC_SVC_FACTORY};

#[test]
fn test_spec_svc_constant_value_happy() {
    assert_eq!(SPEC_SVC, "spec");
}

#[test]
fn test_spec_svc_factory_constant_value_happy() {
    assert_eq!(SPEC_SVC_FACTORY, "spec_factory");
}

#[test]
fn test_spec_svc_factory_constant_not_empty_error() {
    assert!(
        !SPEC_SVC_FACTORY.is_empty(),
        "SPEC_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_spec_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SPEC_SVC_FACTORY.contains(char::is_whitespace),
        "SPEC_SVC_FACTORY must not contain whitespace"
    );
}
