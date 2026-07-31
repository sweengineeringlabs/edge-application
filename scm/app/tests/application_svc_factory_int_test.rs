use edge_application_app::APPLICATION_SVC_FACTORY;

#[test]
fn test_application_svc_factory_constant_value_happy() {
    assert_eq!(APPLICATION_SVC_FACTORY, "application_factory");
}

#[test]
fn test_application_svc_factory_constant_not_empty_error() {
    assert!(!APPLICATION_SVC_FACTORY.is_empty(), "APPLICATION_SVC_FACTORY must not be empty");
}

#[test]
fn test_application_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !APPLICATION_SVC_FACTORY.contains(char::is_whitespace),
        "APPLICATION_SVC_FACTORY must not contain whitespace"
    );
}
