use edge_application_event::EVENT_STORE_SVC_FACTORY;

#[test]
fn test_event_store_svc_factory_constant_value_happy() {
    assert_eq!(EVENT_STORE_SVC_FACTORY, "event_store_factory");
}

#[test]
fn test_event_store_svc_factory_constant_not_empty_error() {
    assert!(!EVENT_STORE_SVC_FACTORY.is_empty(), "EVENT_STORE_SVC_FACTORY must not be empty");
}

#[test]
fn test_event_store_svc_factory_constant_no_whitespace_edge() {
    assert!(!EVENT_STORE_SVC_FACTORY.contains(char::is_whitespace), "EVENT_STORE_SVC_FACTORY must not contain whitespace");
}
