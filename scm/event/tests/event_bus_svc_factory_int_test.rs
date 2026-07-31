use edge_application_event::EVENT_BUS_SVC_FACTORY;

#[test]
fn test_event_bus_svc_factory_constant_value_happy() {
    assert_eq!(EVENT_BUS_SVC_FACTORY, "event_bus_factory");
}

#[test]
fn test_event_bus_svc_factory_constant_not_empty_error() {
    assert!(!EVENT_BUS_SVC_FACTORY.is_empty(), "EVENT_BUS_SVC_FACTORY must not be empty");
}

#[test]
fn test_event_bus_svc_factory_constant_no_whitespace_edge() {
    assert!(!EVENT_BUS_SVC_FACTORY.contains(char::is_whitespace), "EVENT_BUS_SVC_FACTORY must not contain whitespace");
}
