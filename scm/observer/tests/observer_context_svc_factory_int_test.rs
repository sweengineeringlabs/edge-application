use edge_application_observer::OBSERVER_CONTEXT_SVC_FACTORY;

#[test]
fn test_observer_context_svc_factory_constant_value_happy() {
    assert_eq!(OBSERVER_CONTEXT_SVC_FACTORY, "observer_context_factory");
}

#[test]
fn test_observer_context_svc_factory_constant_not_empty_error() {
    assert!(
        !OBSERVER_CONTEXT_SVC_FACTORY.is_empty(),
        "OBSERVER_CONTEXT_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_observer_context_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !OBSERVER_CONTEXT_SVC_FACTORY.contains(char::is_whitespace),
        "OBSERVER_CONTEXT_SVC_FACTORY must not contain whitespace"
    );
}
