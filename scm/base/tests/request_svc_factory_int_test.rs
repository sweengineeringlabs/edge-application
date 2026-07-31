use edge_application_base::REQUEST_SVC_FACTORY;

#[test]
fn test_request_svc_factory_constant_value_happy() {
    assert_eq!(REQUEST_SVC_FACTORY, "request_factory");
}

#[test]
fn test_request_svc_factory_constant_not_empty_error() {
    assert!(
        !REQUEST_SVC_FACTORY.is_empty(),
        "REQUEST_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_request_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !REQUEST_SVC_FACTORY.contains(char::is_whitespace),
        "REQUEST_SVC_FACTORY must not contain whitespace"
    );
}
