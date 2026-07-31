use edge_application_valueobject::{VALUE_OBJECT_SVC, VALUE_OBJECT_SVC_FACTORY};

#[test]
fn test_value_object_svc_constant_value_happy() {
    assert_eq!(VALUE_OBJECT_SVC, "value_object");
}

#[test]
fn test_value_object_svc_factory_constant_value_happy() {
    assert_eq!(VALUE_OBJECT_SVC_FACTORY, "value_object_factory");
}

#[test]
fn test_value_object_svc_factory_constant_not_empty_error() {
    assert!(
        !VALUE_OBJECT_SVC_FACTORY.is_empty(),
        "VALUE_OBJECT_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_value_object_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !VALUE_OBJECT_SVC_FACTORY.contains(char::is_whitespace),
        "VALUE_OBJECT_SVC_FACTORY must not contain whitespace"
    );
}
