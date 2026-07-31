use edge_application_app::APP_BOOTSTRAP_SVC;

#[test]
fn test_bootstrap_svc_constant_value_happy() {
    assert_eq!(APP_BOOTSTRAP_SVC, "app_bootstrap");
}

#[test]
fn test_bootstrap_svc_constant_not_empty_error() {
    assert!(!APP_BOOTSTRAP_SVC.is_empty(), "APP_BOOTSTRAP_SVC must not be empty");
}

#[test]
fn test_bootstrap_svc_constant_no_whitespace_edge() {
    assert!(
        !APP_BOOTSTRAP_SVC.contains(char::is_whitespace),
        "APP_BOOTSTRAP_SVC must not contain whitespace"
    );
}
