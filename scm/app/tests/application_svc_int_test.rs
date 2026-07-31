use edge_application_app::APPLICATION_SVC;

#[test]
fn test_application_svc_constant_value_happy() {
    assert_eq!(APPLICATION_SVC, "application");
}

#[test]
fn test_application_svc_constant_not_empty_error() {
    assert!(!APPLICATION_SVC.is_empty(), "APPLICATION_SVC must not be empty");
}

#[test]
fn test_application_svc_constant_no_whitespace_edge() {
    assert!(
        !APPLICATION_SVC.contains(char::is_whitespace),
        "APPLICATION_SVC must not contain whitespace"
    );
}
