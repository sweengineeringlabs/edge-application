use edge_application_observer::{
    HANDLER_TRACER_SVC, LOG_DRAIN_SVC, METRIC_REGISTRY_SVC, OBSERVE_FACTORY_SVC,
};

#[test]
fn test_handler_tracer_svc_constant_is_non_empty() {
    assert!(!HANDLER_TRACER_SVC.is_empty());
}

#[test]
fn test_metric_registry_svc_constant_is_non_empty() {
    assert!(!METRIC_REGISTRY_SVC.is_empty());
}

#[test]
fn test_log_drain_svc_constant_is_non_empty() {
    assert!(!LOG_DRAIN_SVC.is_empty());
}

#[test]
fn test_observe_factory_svc_constant_is_non_empty() {
    assert!(!OBSERVE_FACTORY_SVC.is_empty());
}

#[test]
fn test_svc_constants_are_unique() {
    let keys = [
        HANDLER_TRACER_SVC,
        METRIC_REGISTRY_SVC,
        LOG_DRAIN_SVC,
        OBSERVE_FACTORY_SVC,
    ];
    let mut seen = std::collections::HashSet::new();
    for k in &keys {
        assert!(seen.insert(*k), "duplicate service key: {k}");
    }
}
