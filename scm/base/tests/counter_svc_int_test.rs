use edge_application_base::COUNTER_SVC;

#[test]
fn test_counter_svc_constant_value_happy() {
    assert_eq!(COUNTER_SVC, "counter");
}

#[test]
fn test_counter_svc_constant_not_empty_error() {
    assert!(!COUNTER_SVC.is_empty(), "COUNTER_SVC must not be empty");
}

#[test]
fn test_counter_svc_constant_no_whitespace_edge() {
    assert!(
        !COUNTER_SVC.contains(char::is_whitespace),
        "COUNTER_SVC must not contain whitespace"
    );
}
