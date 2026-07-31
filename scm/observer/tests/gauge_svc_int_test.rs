use edge_application_observer::GAUGE_SVC;

#[test]
fn test_gauge_svc_key_non_empty_happy() {
    assert!(!GAUGE_SVC.is_empty());
}

#[test]
fn test_gauge_svc_key_has_edge_prefix_error() {
    assert!(
        GAUGE_SVC.starts_with("edge."),
        "service key must be namespaced under edge.*"
    );
}

#[test]
fn test_gauge_svc_key_stable_across_calls_edge() {
    assert_eq!(GAUGE_SVC, "edge.observe.gauge");
}
