use edge_application_base::SPAN_SVC;

#[test]
fn test_span_svc_constant_value_happy() {
    assert_eq!(SPAN_SVC, "span");
}

#[test]
fn test_span_svc_constant_not_empty_error() {
    assert!(!SPAN_SVC.is_empty(), "SPAN_SVC must not be empty");
}

#[test]
fn test_span_svc_constant_no_whitespace_edge() {
    assert!(
        !SPAN_SVC.contains(char::is_whitespace),
        "SPAN_SVC must not contain whitespace"
    );
}
