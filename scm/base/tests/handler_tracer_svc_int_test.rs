use edge_application_base::HANDLER_TRACER_SVC;

#[test]
fn test_handler_tracer_svc_constant_value_happy() {
    assert_eq!(HANDLER_TRACER_SVC, "handler_tracer");
}

#[test]
fn test_handler_tracer_svc_constant_not_empty_error() {
    assert!(!HANDLER_TRACER_SVC.is_empty(), "HANDLER_TRACER_SVC must not be empty");
}

#[test]
fn test_handler_tracer_svc_constant_no_whitespace_edge() {
    assert!(
        !HANDLER_TRACER_SVC.contains(char::is_whitespace),
        "HANDLER_TRACER_SVC must not contain whitespace"
    );
}
