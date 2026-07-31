use edge_application_snapshot::{SNAPSHOT_STORE_SVC, SNAPSHOT_STORE_SVC_FACTORY};

#[test]
fn test_snapshot_store_svc_constant_value_happy() {
    assert_eq!(SNAPSHOT_STORE_SVC, "snapshot_store");
}

#[test]
fn test_snapshot_store_svc_factory_constant_value_happy() {
    assert_eq!(SNAPSHOT_STORE_SVC_FACTORY, "snapshot_store_factory");
}

#[test]
fn test_snapshot_store_svc_factory_constant_not_empty_error() {
    assert!(
        !SNAPSHOT_STORE_SVC_FACTORY.is_empty(),
        "SNAPSHOT_STORE_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_snapshot_store_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SNAPSHOT_STORE_SVC_FACTORY.contains(char::is_whitespace),
        "SNAPSHOT_STORE_SVC_FACTORY must not contain whitespace"
    );
}
