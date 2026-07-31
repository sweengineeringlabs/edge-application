use edge_application_app::APP_BOOTSTRAP_SVC_FACTORY;

#[test]
fn test_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(APP_BOOTSTRAP_SVC_FACTORY, "app_bootstrap_factory");
}

#[test]
fn test_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(!APP_BOOTSTRAP_SVC_FACTORY.is_empty(), "APP_BOOTSTRAP_SVC_FACTORY must not be empty");
}

#[test]
fn test_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !APP_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "APP_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
