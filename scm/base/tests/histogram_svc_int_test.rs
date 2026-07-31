use edge_application_base::HISTOGRAM_SVC;

#[test]
fn test_histogram_svc_constant_value_happy() {
    assert_eq!(HISTOGRAM_SVC, "histogram");
}

#[test]
fn test_histogram_svc_constant_not_empty_error() {
    assert!(!HISTOGRAM_SVC.is_empty(), "HISTOGRAM_SVC must not be empty");
}

#[test]
fn test_histogram_svc_constant_no_whitespace_edge() {
    assert!(
        !HISTOGRAM_SVC.contains(char::is_whitespace),
        "HISTOGRAM_SVC must not contain whitespace"
    );
}
