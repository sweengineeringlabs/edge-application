use edge_application_projection::{PROJECTION_SVC, PROJECTION_SVC_FACTORY};

#[test]
fn test_projection_svc_constant_value_happy() {
    assert_eq!(PROJECTION_SVC, "projection");
}

#[test]
fn test_projection_svc_factory_constant_value_happy() {
    assert_eq!(PROJECTION_SVC_FACTORY, "projection_factory");
}

#[test]
fn test_projection_svc_factory_constant_not_empty_error() {
    assert!(!PROJECTION_SVC_FACTORY.is_empty(), "PROJECTION_SVC_FACTORY must not be empty");
}

#[test]
fn test_projection_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !PROJECTION_SVC_FACTORY.contains(char::is_whitespace),
        "PROJECTION_SVC_FACTORY must not contain whitespace"
    );
}
