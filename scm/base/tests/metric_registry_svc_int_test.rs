use edge_application_base::METRIC_REGISTRY_SVC;

#[test]
fn test_metric_registry_svc_constant_value_happy() {
    assert_eq!(METRIC_REGISTRY_SVC, "metric_registry");
}

#[test]
fn test_metric_registry_svc_constant_not_empty_error() {
    assert!(!METRIC_REGISTRY_SVC.is_empty(), "METRIC_REGISTRY_SVC must not be empty");
}

#[test]
fn test_metric_registry_svc_constant_no_whitespace_edge() {
    assert!(
        !METRIC_REGISTRY_SVC.contains(char::is_whitespace),
        "METRIC_REGISTRY_SVC must not contain whitespace"
    );
}
