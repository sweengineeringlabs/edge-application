use edge_application_base::GAUGE_SVC;

#[test]
fn test_gauge_svc_constant_value_happy() {
    assert_eq!(GAUGE_SVC, "gauge");
}

#[test]
fn test_gauge_svc_constant_not_empty_error() {
    assert!(!GAUGE_SVC.is_empty(), "GAUGE_SVC must not be empty");
}

#[test]
fn test_gauge_svc_constant_no_whitespace_edge() {
    assert!(
        !GAUGE_SVC.contains(char::is_whitespace),
        "GAUGE_SVC must not contain whitespace"
    );
}
