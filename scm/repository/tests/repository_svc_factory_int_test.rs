use edge_application_repository::{REPOSITORY_SVC, REPOSITORY_SVC_FACTORY};

#[test]
fn test_repository_svc_constant_value_happy() {
    assert_eq!(REPOSITORY_SVC, "repository");
}

#[test]
fn test_repository_svc_factory_constant_value_happy() {
    assert_eq!(REPOSITORY_SVC_FACTORY, "repository_factory");
}

#[test]
fn test_repository_svc_factory_constant_not_empty_error() {
    assert!(
        !REPOSITORY_SVC_FACTORY.is_empty(),
        "REPOSITORY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_repository_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !REPOSITORY_SVC_FACTORY.contains(char::is_whitespace),
        "REPOSITORY_SVC_FACTORY must not contain whitespace"
    );
}
