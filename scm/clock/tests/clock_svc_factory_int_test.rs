use edge_application_clock::CLOCK_SVC_FACTORY;

#[test]
fn test_clock_svc_factory_constant_value_happy() {
    assert_eq!(CLOCK_SVC_FACTORY, "clock_factory");
}

#[test]
fn test_clock_svc_factory_constant_not_empty_error() {
    assert!(!CLOCK_SVC_FACTORY.is_empty(), "CLOCK_SVC_FACTORY must not be empty");
}

#[test]
fn test_clock_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !CLOCK_SVC_FACTORY.contains(char::is_whitespace),
        "CLOCK_SVC_FACTORY must not contain whitespace"
    );
}
