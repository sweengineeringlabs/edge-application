use edge_application_lifecycle::{LIFECYCLE_SVC, LIFECYCLE_SVC_FACTORY};

#[test]
fn test_lifecycle_svc_constant_value_happy() {
    assert_eq!(LIFECYCLE_SVC, "lifecycle");
}

#[test]
fn test_lifecycle_svc_factory_constant_value_happy() {
    assert_eq!(LIFECYCLE_SVC_FACTORY, "lifecycle_factory");
}

#[test]
fn test_lifecycle_svc_factory_constant_not_empty_error() {
    assert!(!LIFECYCLE_SVC_FACTORY.is_empty(), "LIFECYCLE_SVC_FACTORY must not be empty");
}

#[test]
fn test_lifecycle_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !LIFECYCLE_SVC_FACTORY.contains(char::is_whitespace),
        "LIFECYCLE_SVC_FACTORY must not contain whitespace"
    );
}
