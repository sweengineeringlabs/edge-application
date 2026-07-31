use edge_application_saga::{SAGA_EVENT_SVC, SAGA_EVENT_SVC_FACTORY};

#[test]
fn test_saga_event_svc_constant_value_happy() {
    assert_eq!(SAGA_EVENT_SVC, "saga_event");
}

#[test]
fn test_saga_event_svc_factory_constant_value_happy() {
    assert_eq!(SAGA_EVENT_SVC_FACTORY, "saga_event_factory");
}

#[test]
fn test_saga_event_svc_factory_constant_not_empty_error() {
    assert!(
        !SAGA_EVENT_SVC_FACTORY.is_empty(),
        "SAGA_EVENT_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_saga_event_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SAGA_EVENT_SVC_FACTORY.contains(char::is_whitespace),
        "SAGA_EVENT_SVC_FACTORY must not contain whitespace"
    );
}
