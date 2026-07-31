use edge_application_base::RESPONSE_SVC;

#[test]
fn test_response_svc_constant_value_happy() {
    assert_eq!(RESPONSE_SVC, "response");
}

#[test]
fn test_response_svc_constant_not_empty_error() {
    assert!(!RESPONSE_SVC.is_empty(), "RESPONSE_SVC must not be empty");
}

#[test]
fn test_response_svc_constant_no_whitespace_edge() {
    assert!(
        !RESPONSE_SVC.contains(char::is_whitespace),
        "RESPONSE_SVC must not contain whitespace"
    );
}
