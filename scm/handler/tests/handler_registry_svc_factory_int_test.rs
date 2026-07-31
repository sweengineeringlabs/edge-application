use edge_application_handler::HANDLER_REGISTRY_SVC_FACTORY;

#[test]
fn test_handler_registry_svc_factory_constant_value_happy() {
    assert_eq!(HANDLER_REGISTRY_SVC_FACTORY, "handler_registry_factory");
}

#[test]
fn test_handler_registry_svc_factory_constant_not_empty_error() {
    assert!(
        !HANDLER_REGISTRY_SVC_FACTORY.is_empty(),
        "HANDLER_REGISTRY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_handler_registry_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !HANDLER_REGISTRY_SVC_FACTORY.contains(char::is_whitespace),
        "HANDLER_REGISTRY_SVC_FACTORY must not contain whitespace"
    );
}
