use edge_application_base::OBSERVER_CONTEXT_SVC;

#[test]
fn test_observer_context_svc_constant_value_happy() {
    assert_eq!(OBSERVER_CONTEXT_SVC, "observer_context");
}

#[test]
fn test_observer_context_svc_constant_not_empty_error() {
    assert!(!OBSERVER_CONTEXT_SVC.is_empty(), "OBSERVER_CONTEXT_SVC must not be empty");
}

#[test]
fn test_observer_context_svc_constant_no_whitespace_edge() {
    assert!(
        !OBSERVER_CONTEXT_SVC.contains(char::is_whitespace),
        "OBSERVER_CONTEXT_SVC must not contain whitespace"
    );
}
