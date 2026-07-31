use edge_application_policy::{POLICY_SVC, POLICY_SVC_FACTORY};

#[test]
fn test_policy_svc_anchor_is_unit_happy() {
    assert_eq!(POLICY_SVC, (), "POLICY_SVC is a Rule-221 anchor and must be the unit value");
}

#[test]
fn test_policy_svc_factory_constant_value_happy() {
    assert_eq!(POLICY_SVC_FACTORY, "policy_factory");
}

#[test]
fn test_policy_svc_factory_constant_not_empty_error() {
    assert!(!POLICY_SVC_FACTORY.is_empty(), "POLICY_SVC_FACTORY must not be empty");
}

#[test]
fn test_policy_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !POLICY_SVC_FACTORY.contains(char::is_whitespace),
        "POLICY_SVC_FACTORY must not contain whitespace"
    );
}
