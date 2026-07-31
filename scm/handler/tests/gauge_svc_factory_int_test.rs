use edge_application_handler::{GAUGE_SVC, GAUGE_SVC_FACTORY};

#[test]
fn test_gauge_svc_constant_value_happy() {
    assert_eq!(GAUGE_SVC, "gauge");
}

#[test]
fn test_gauge_svc_factory_constant_value_happy() {
    assert_eq!(GAUGE_SVC_FACTORY, "gauge_factory");
}

#[test]
fn test_gauge_svc_factory_constant_not_empty_error() {
    assert!(
        !GAUGE_SVC_FACTORY.is_empty(),
        "GAUGE_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_gauge_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !GAUGE_SVC_FACTORY.contains(char::is_whitespace),
        "GAUGE_SVC_FACTORY must not contain whitespace"
    );
}
