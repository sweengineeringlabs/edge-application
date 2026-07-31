use edge_application_handler::HANDLER_SVC_FACTORY;

#[test]
fn test_handler_svc_factory_constant_value_happy() {
    assert_eq!(HANDLER_SVC_FACTORY, "handler_factory");
}

#[test]
fn test_handler_svc_factory_constant_not_empty_error() {
    assert!(
        !HANDLER_SVC_FACTORY.is_empty(),
        "HANDLER_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_handler_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !HANDLER_SVC_FACTORY.contains(char::is_whitespace),
        "HANDLER_SVC_FACTORY must not contain whitespace"
    );
}
