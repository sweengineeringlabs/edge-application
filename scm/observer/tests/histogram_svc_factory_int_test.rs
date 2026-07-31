use edge_application_observer::HISTOGRAM_SVC_FACTORY;

#[test]
fn test_histogram_svc_factory_constant_value_happy() {
    assert_eq!(HISTOGRAM_SVC_FACTORY, "histogram_factory");
}

#[test]
fn test_histogram_svc_factory_constant_not_empty_error() {
    assert!(!HISTOGRAM_SVC_FACTORY.is_empty(), "HISTOGRAM_SVC_FACTORY must not be empty");
}

#[test]
fn test_histogram_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !HISTOGRAM_SVC_FACTORY.contains(char::is_whitespace),
        "HISTOGRAM_SVC_FACTORY must not contain whitespace"
    );
}
