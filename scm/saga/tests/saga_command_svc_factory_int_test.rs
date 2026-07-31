use edge_application_saga::{SAGA_COMMAND_SVC, SAGA_COMMAND_SVC_FACTORY};

#[test]
fn test_saga_command_svc_constant_value_happy() {
    assert_eq!(SAGA_COMMAND_SVC, "saga_command");
}

#[test]
fn test_saga_command_svc_factory_constant_value_happy() {
    assert_eq!(SAGA_COMMAND_SVC_FACTORY, "saga_command_factory");
}

#[test]
fn test_saga_command_svc_factory_constant_not_empty_error() {
    assert!(
        !SAGA_COMMAND_SVC_FACTORY.is_empty(),
        "SAGA_COMMAND_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_saga_command_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SAGA_COMMAND_SVC_FACTORY.contains(char::is_whitespace),
        "SAGA_COMMAND_SVC_FACTORY must not contain whitespace"
    );
}
