use edge_application_observer::NOOP_OBSERVE_SVC_FACTORY;

#[test]
fn test_noop_observe_svc_factory_constant_value_happy() {
    assert_eq!(NOOP_OBSERVE_SVC_FACTORY, "noop_observe_factory");
}

#[test]
fn test_noop_observe_svc_factory_constant_not_empty_error() {
    assert!(!NOOP_OBSERVE_SVC_FACTORY.is_empty(), "NOOP_OBSERVE_SVC_FACTORY must not be empty");
}

#[test]
fn test_noop_observe_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !NOOP_OBSERVE_SVC_FACTORY.contains(char::is_whitespace),
        "NOOP_OBSERVE_SVC_FACTORY must not contain whitespace"
    );
}
