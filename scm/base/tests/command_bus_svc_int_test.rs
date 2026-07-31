use edge_application_base::COMMAND_BUS_SVC;

#[test]
fn test_command_bus_svc_constant_value_happy() {
    assert_eq!(COMMAND_BUS_SVC, "command_bus");
}

#[test]
fn test_command_bus_svc_constant_not_empty_error() {
    assert!(!COMMAND_BUS_SVC.is_empty(), "COMMAND_BUS_SVC must not be empty");
}

#[test]
fn test_command_bus_svc_constant_no_whitespace_edge() {
    assert!(
        !COMMAND_BUS_SVC.contains(char::is_whitespace),
        "COMMAND_BUS_SVC must not contain whitespace"
    );
}
