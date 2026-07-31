use edge_application_base::COMMAND_BUS_SVC_FACTORY;

#[test]
fn test_command_bus_svc_factory_constant_value_happy() {
    assert_eq!(COMMAND_BUS_SVC_FACTORY, "command_bus_factory");
}

#[test]
fn test_command_bus_svc_factory_constant_not_empty_error() {
    assert!(!COMMAND_BUS_SVC_FACTORY.is_empty(), "COMMAND_BUS_SVC_FACTORY must not be empty");
}

#[test]
fn test_command_bus_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !COMMAND_BUS_SVC_FACTORY.contains(char::is_whitespace),
        "COMMAND_BUS_SVC_FACTORY must not contain whitespace"
    );
}
