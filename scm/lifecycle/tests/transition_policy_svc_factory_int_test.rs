use edge_application_lifecycle::{TRANSITION_POLICY_SVC, TRANSITION_POLICY_SVC_FACTORY};

#[test]
fn test_transition_policy_svc_constant_value_happy() {
    assert_eq!(TRANSITION_POLICY_SVC, "transition_policy");
}

#[test]
fn test_transition_policy_svc_factory_constant_value_happy() {
    assert_eq!(TRANSITION_POLICY_SVC_FACTORY, "transition_policy_factory");
}

#[test]
fn test_transition_policy_svc_factory_constant_not_empty_error() {
    assert!(
        !TRANSITION_POLICY_SVC_FACTORY.is_empty(),
        "TRANSITION_POLICY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_transition_policy_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !TRANSITION_POLICY_SVC_FACTORY.contains(char::is_whitespace),
        "TRANSITION_POLICY_SVC_FACTORY must not contain whitespace"
    );
}
