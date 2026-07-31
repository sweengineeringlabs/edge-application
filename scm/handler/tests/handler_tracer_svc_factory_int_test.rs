use edge_application_handler::{HANDLER_TRACER_SVC, HANDLER_TRACER_SVC_FACTORY};

#[test]
fn test_handler_tracer_svc_constant_value_happy() {
    assert_eq!(HANDLER_TRACER_SVC, "handler_tracer");
}

#[test]
fn test_handler_tracer_svc_factory_constant_value_happy() {
    assert_eq!(HANDLER_TRACER_SVC_FACTORY, "handler_tracer_factory");
}

#[test]
fn test_handler_tracer_svc_factory_constant_not_empty_error() {
    assert!(
        !HANDLER_TRACER_SVC_FACTORY.is_empty(),
        "HANDLER_TRACER_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_handler_tracer_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !HANDLER_TRACER_SVC_FACTORY.contains(char::is_whitespace),
        "HANDLER_TRACER_SVC_FACTORY must not contain whitespace"
    );
}
