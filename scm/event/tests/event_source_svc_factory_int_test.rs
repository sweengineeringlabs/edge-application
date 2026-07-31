use edge_application_event::EVENT_SOURCE_SVC_FACTORY;

#[test]
fn test_event_source_svc_factory_constant_value_happy() {
    assert_eq!(EVENT_SOURCE_SVC_FACTORY, "event_source_factory");
}

#[test]
fn test_event_source_svc_factory_constant_not_empty_error() {
    assert!(!EVENT_SOURCE_SVC_FACTORY.is_empty(), "EVENT_SOURCE_SVC_FACTORY must not be empty");
}

#[test]
fn test_event_source_svc_factory_constant_no_whitespace_edge() {
    assert!(!EVENT_SOURCE_SVC_FACTORY.contains(char::is_whitespace), "EVENT_SOURCE_SVC_FACTORY must not contain whitespace");
}
