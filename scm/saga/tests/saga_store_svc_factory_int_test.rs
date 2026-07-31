use edge_application_saga::{SAGA_STORE_SVC, SAGA_STORE_SVC_FACTORY};

#[test]
fn test_saga_store_svc_constant_value_happy() {
    assert_eq!(SAGA_STORE_SVC, "saga_store");
}

#[test]
fn test_saga_store_svc_factory_constant_value_happy() {
    assert_eq!(SAGA_STORE_SVC_FACTORY, "saga_store_factory");
}

#[test]
fn test_saga_store_svc_factory_constant_not_empty_error() {
    assert!(
        !SAGA_STORE_SVC_FACTORY.is_empty(),
        "SAGA_STORE_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_saga_store_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SAGA_STORE_SVC_FACTORY.contains(char::is_whitespace),
        "SAGA_STORE_SVC_FACTORY must not contain whitespace"
    );
}
