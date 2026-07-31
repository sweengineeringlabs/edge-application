use edge_application_observer::HISTOGRAM_SVC;

#[test]
fn test_histogram_svc_key_non_empty_happy() {
    assert!(!HISTOGRAM_SVC.is_empty());
}

#[test]
fn test_histogram_svc_key_has_edge_prefix_error() {
    assert!(
        HISTOGRAM_SVC.starts_with("edge."),
        "service key must be namespaced under edge.*"
    );
}

#[test]
fn test_histogram_svc_key_stable_across_calls_edge() {
    assert_eq!(HISTOGRAM_SVC, "edge.observe.histogram");
}
