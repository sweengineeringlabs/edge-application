use edge_application_repository::{QUERYABLE_REPOSITORY_SVC, QUERYABLE_REPOSITORY_SVC_FACTORY};

#[test]
fn test_queryable_repository_svc_constant_value_happy() {
    assert_eq!(QUERYABLE_REPOSITORY_SVC, "queryable_repository");
}

#[test]
fn test_queryable_repository_svc_factory_constant_value_happy() {
    assert_eq!(
        QUERYABLE_REPOSITORY_SVC_FACTORY,
        "queryable_repository_factory"
    );
}

#[test]
fn test_queryable_repository_svc_factory_constant_not_empty_error() {
    assert!(
        !QUERYABLE_REPOSITORY_SVC_FACTORY.is_empty(),
        "QUERYABLE_REPOSITORY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_queryable_repository_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !QUERYABLE_REPOSITORY_SVC_FACTORY.contains(char::is_whitespace),
        "QUERYABLE_REPOSITORY_SVC_FACTORY must not contain whitespace"
    );
}
