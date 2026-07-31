use edge_application_event::AGGREGATE_SVC_FACTORY;

#[test]
fn test_aggregate_svc_factory_constant_value_happy() {
    assert_eq!(AGGREGATE_SVC_FACTORY, "aggregate_factory");
}

#[test]
fn test_aggregate_svc_factory_constant_not_empty_error() {
    assert!(!AGGREGATE_SVC_FACTORY.is_empty(), "AGGREGATE_SVC_FACTORY must not be empty");
}

#[test]
fn test_aggregate_svc_factory_constant_no_whitespace_edge() {
    assert!(!AGGREGATE_SVC_FACTORY.contains(char::is_whitespace), "AGGREGATE_SVC_FACTORY must not contain whitespace");
}
