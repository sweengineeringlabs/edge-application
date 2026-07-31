use edge_application_base::REQUEST_SVC;

#[test]
fn test_request_svc_constant_value_happy() {
    assert_eq!(REQUEST_SVC, "request");
}

#[test]
fn test_request_svc_constant_not_empty_error() {
    assert!(!REQUEST_SVC.is_empty(), "REQUEST_SVC must not be empty");
}

#[test]
fn test_request_svc_constant_no_whitespace_edge() {
    assert!(
        !REQUEST_SVC.contains(char::is_whitespace),
        "REQUEST_SVC must not contain whitespace"
    );
}
