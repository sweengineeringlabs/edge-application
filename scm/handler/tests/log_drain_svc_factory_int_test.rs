use edge_application_handler::{LOG_DRAIN_SVC, LOG_DRAIN_SVC_FACTORY};

#[test]
fn test_log_drain_svc_constant_value_happy() {
    assert_eq!(LOG_DRAIN_SVC, "log_drain");
}

#[test]
fn test_log_drain_svc_factory_constant_value_happy() {
    assert_eq!(LOG_DRAIN_SVC_FACTORY, "log_drain_factory");
}

#[test]
fn test_log_drain_svc_factory_constant_not_empty_error() {
    assert!(
        !LOG_DRAIN_SVC_FACTORY.is_empty(),
        "LOG_DRAIN_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_log_drain_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !LOG_DRAIN_SVC_FACTORY.contains(char::is_whitespace),
        "LOG_DRAIN_SVC_FACTORY must not contain whitespace"
    );
}
