use edge_application_projection::{PROJECTION_EVENT_SVC, PROJECTION_EVENT_SVC_FACTORY};

#[test]
fn test_projection_event_svc_constant_value_happy() {
    assert_eq!(PROJECTION_EVENT_SVC, "projection_event");
}

#[test]
fn test_projection_event_svc_factory_constant_value_happy() {
    assert_eq!(PROJECTION_EVENT_SVC_FACTORY, "projection_event_factory");
}

#[test]
fn test_projection_event_svc_factory_constant_not_empty_error() {
    assert!(
        !PROJECTION_EVENT_SVC_FACTORY.is_empty(),
        "PROJECTION_EVENT_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_projection_event_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !PROJECTION_EVENT_SVC_FACTORY.contains(char::is_whitespace),
        "PROJECTION_EVENT_SVC_FACTORY must not contain whitespace"
    );
}
