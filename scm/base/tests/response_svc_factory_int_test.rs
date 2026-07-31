use edge_application_base::RESPONSE_SVC_FACTORY;

#[test]
fn test_response_svc_factory_constant_value_happy() {
    assert_eq!(RESPONSE_SVC_FACTORY, "response_factory");
}

#[test]
fn test_response_svc_factory_constant_not_empty_error() {
    assert!(
        !RESPONSE_SVC_FACTORY.is_empty(),
        "RESPONSE_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_response_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !RESPONSE_SVC_FACTORY.contains(char::is_whitespace),
        "RESPONSE_SVC_FACTORY must not contain whitespace"
    );
}
