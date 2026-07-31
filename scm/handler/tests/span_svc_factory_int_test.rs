use edge_application_handler::{SPAN_SVC, SPAN_SVC_FACTORY};

#[test]
fn test_span_svc_constant_value_happy() {
    assert_eq!(SPAN_SVC, "span");
}

#[test]
fn test_span_svc_factory_constant_value_happy() {
    assert_eq!(SPAN_SVC_FACTORY, "span_factory");
}

#[test]
fn test_span_svc_factory_constant_not_empty_error() {
    assert!(
        !SPAN_SVC_FACTORY.is_empty(),
        "SPAN_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_span_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SPAN_SVC_FACTORY.contains(char::is_whitespace),
        "SPAN_SVC_FACTORY must not contain whitespace"
    );
}
