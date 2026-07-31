use edge_application_validator::VALIDATOR_SVC_FACTORY;

#[test]
fn test_validator_svc_factory_constant_value_happy() {
    assert_eq!(VALIDATOR_SVC_FACTORY, "validator_factory");
}

#[test]
fn test_validator_svc_factory_constant_not_empty_error() {
    assert!(
        !VALIDATOR_SVC_FACTORY.is_empty(),
        "VALIDATOR_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_validator_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !VALIDATOR_SVC_FACTORY.contains(char::is_whitespace),
        "VALIDATOR_SVC_FACTORY must not contain whitespace"
    );
}
