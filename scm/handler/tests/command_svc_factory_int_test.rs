use edge_application_handler::{COMMAND_SVC, COMMAND_SVC_FACTORY};

#[test]
fn test_command_svc_constant_value_happy() {
    assert_eq!(COMMAND_SVC, "command");
}

#[test]
fn test_command_svc_factory_constant_value_happy() {
    assert_eq!(COMMAND_SVC_FACTORY, "command_factory");
}

#[test]
fn test_command_svc_factory_constant_not_empty_error() {
    assert!(
        !COMMAND_SVC_FACTORY.is_empty(),
        "COMMAND_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_command_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !COMMAND_SVC_FACTORY.contains(char::is_whitespace),
        "COMMAND_SVC_FACTORY must not contain whitespace"
    );
}
