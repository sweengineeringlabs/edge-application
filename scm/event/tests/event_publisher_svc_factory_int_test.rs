use edge_application_event::EVENT_PUBLISHER_SVC_FACTORY;

#[test]
fn test_event_publisher_svc_factory_constant_value_happy() {
    assert_eq!(EVENT_PUBLISHER_SVC_FACTORY, "event_publisher_factory");
}

#[test]
fn test_event_publisher_svc_factory_constant_not_empty_error() {
    assert!(!EVENT_PUBLISHER_SVC_FACTORY.is_empty(), "EVENT_PUBLISHER_SVC_FACTORY must not be empty");
}

#[test]
fn test_event_publisher_svc_factory_constant_no_whitespace_edge() {
    assert!(!EVENT_PUBLISHER_SVC_FACTORY.contains(char::is_whitespace), "EVENT_PUBLISHER_SVC_FACTORY must not contain whitespace");
}
