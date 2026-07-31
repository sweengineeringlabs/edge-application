use edge_application_event::DOMAIN_EVENT_SVC_FACTORY;

#[test]
fn test_domain_event_svc_factory_constant_value_happy() {
    assert_eq!(DOMAIN_EVENT_SVC_FACTORY, "domain_event_factory");
}

#[test]
fn test_domain_event_svc_factory_constant_not_empty_error() {
    assert!(!DOMAIN_EVENT_SVC_FACTORY.is_empty(), "DOMAIN_EVENT_SVC_FACTORY must not be empty");
}

#[test]
fn test_domain_event_svc_factory_constant_no_whitespace_edge() {
    assert!(!DOMAIN_EVENT_SVC_FACTORY.contains(char::is_whitespace), "DOMAIN_EVENT_SVC_FACTORY must not contain whitespace");
}
