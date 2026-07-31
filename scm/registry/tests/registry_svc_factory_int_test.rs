use edge_application_registry::REGISTRY_SVC_FACTORY;

#[test]
fn test_registry_svc_factory_constant_value_happy() {
    assert_eq!(REGISTRY_SVC_FACTORY, "registry_factory");
}

#[test]
fn test_registry_svc_factory_constant_not_empty_error() {
    assert!(
        !REGISTRY_SVC_FACTORY.is_empty(),
        "REGISTRY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_registry_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !REGISTRY_SVC_FACTORY.contains(char::is_whitespace),
        "REGISTRY_SVC_FACTORY must not contain whitespace"
    );
}
