use edge_application_entity::ENTITY_SVC_FACTORY;

#[test]
fn test_entity_svc_factory_constant_value_happy() {
    assert_eq!(ENTITY_SVC_FACTORY, "entity_factory");
}

#[test]
fn test_entity_svc_factory_constant_not_empty_error() {
    assert!(
        !ENTITY_SVC_FACTORY.is_empty(),
        "ENTITY_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_entity_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !ENTITY_SVC_FACTORY.contains(char::is_whitespace),
        "ENTITY_SVC_FACTORY must not contain whitespace"
    );
}
