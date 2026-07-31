use edge_application_observer::COUNTER_SVC_FACTORY;

#[test]
fn test_counter_svc_factory_constant_value_happy() {
    assert_eq!(COUNTER_SVC_FACTORY, "counter_factory");
}

#[test]
fn test_counter_svc_factory_constant_not_empty_error() {
    assert!(!COUNTER_SVC_FACTORY.is_empty(), "COUNTER_SVC_FACTORY must not be empty");
}

#[test]
fn test_counter_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !COUNTER_SVC_FACTORY.contains(char::is_whitespace),
        "COUNTER_SVC_FACTORY must not contain whitespace"
    );
}
