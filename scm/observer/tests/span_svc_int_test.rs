use edge_application_observer::SPAN_SVC;

#[test]
fn test_span_svc_key_non_empty_happy() {
    assert!(!SPAN_SVC.is_empty());
}

#[test]
fn test_span_svc_key_has_edge_prefix_error() {
    assert!(
        SPAN_SVC.starts_with("edge."),
        "service key must be namespaced under edge.*"
    );
}

#[test]
fn test_span_svc_key_stable_across_calls_edge() {
    assert_eq!(SPAN_SVC, "edge.observe.span");
}
