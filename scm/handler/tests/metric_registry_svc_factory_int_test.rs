use edge_application_handler::{METRIC_REGISTRY_SVC, METRIC_REGISTRY_SVC_FACTORY};

#[test]
fn test_metric_registry_svc_constant_value_happy() {
    assert_eq!(METRIC_REGISTRY_SVC, "metric_registry");
}

#[test]
fn test_metric_registry_svc_factory_constant_value_happy() {
    assert_eq!(METRIC_REGISTRY_SVC_FACTORY, "metric_registry_factory");
}

#[test]
fn test_metric_registry_svc_factory_constant_not_empty_error() {
    assert!(
        !METRIC_REGISTRY_SVC_FACTORY.is_empty(),
        "METRIC_REGISTRY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_metric_registry_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !METRIC_REGISTRY_SVC_FACTORY.contains(char::is_whitespace),
        "METRIC_REGISTRY_SVC_FACTORY must not contain whitespace"
    );
}
