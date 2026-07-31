use edge_application_base::LOG_DRAIN_SVC;

#[test]
fn test_log_drain_svc_constant_value_happy() {
    assert_eq!(LOG_DRAIN_SVC, "log_drain");
}

#[test]
fn test_log_drain_svc_constant_not_empty_error() {
    assert!(!LOG_DRAIN_SVC.is_empty(), "LOG_DRAIN_SVC must not be empty");
}

#[test]
fn test_log_drain_svc_constant_no_whitespace_edge() {
    assert!(
        !LOG_DRAIN_SVC.contains(char::is_whitespace),
        "LOG_DRAIN_SVC must not contain whitespace"
    );
}
