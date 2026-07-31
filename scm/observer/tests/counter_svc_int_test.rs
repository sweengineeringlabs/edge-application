use edge_application_observer::COUNTER_SVC;

#[test]
fn test_counter_svc_key_non_empty_happy() {
    assert!(!COUNTER_SVC.is_empty());
}

#[test]
fn test_counter_svc_key_has_edge_prefix_error() {
    assert!(
        COUNTER_SVC.starts_with("edge."),
        "service key must be namespaced under edge.*"
    );
}

#[test]
fn test_counter_svc_key_stable_across_calls_edge() {
    let key1 = COUNTER_SVC;
    let key2 = COUNTER_SVC;
    assert_eq!(key1, key2, "COUNTER_SVC must be stable");
    assert_eq!(key1, "edge.observe.counter", "COUNTER_SVC must have expected value");
}
