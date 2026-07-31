use edge_application_query::{QUERY_SVC, QUERY_SVC_FACTORY};

#[test]
fn test_query_svc_constant_value_happy() {
    assert_eq!(QUERY_SVC, "query");
}

#[test]
fn test_query_svc_factory_constant_value_happy() {
    assert_eq!(QUERY_SVC_FACTORY, "query_factory");
}

#[test]
fn test_query_svc_factory_constant_not_empty_error() {
    assert!(!QUERY_SVC_FACTORY.is_empty(), "QUERY_SVC_FACTORY must not be empty");
}

#[test]
fn test_query_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !QUERY_SVC_FACTORY.contains(char::is_whitespace),
        "QUERY_SVC_FACTORY must not contain whitespace"
    );
}
