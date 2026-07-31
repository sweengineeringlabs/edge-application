use edge_application_observer::OBSERVE_BOOTSTRAP_SVC_FACTORY;

#[test]
fn test_observe_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(OBSERVE_BOOTSTRAP_SVC_FACTORY, "observe_bootstrap_factory");
}

#[test]
fn test_observe_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(
        !OBSERVE_BOOTSTRAP_SVC_FACTORY.is_empty(),
        "OBSERVE_BOOTSTRAP_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_observe_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !OBSERVE_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "OBSERVE_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
