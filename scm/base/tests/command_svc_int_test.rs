use edge_application_base::COMMAND_SVC;

#[test]
fn test_command_svc_constant_value_happy() {
    assert_eq!(COMMAND_SVC, "command");
}

#[test]
fn test_command_svc_constant_not_empty_error() {
    assert!(!COMMAND_SVC.is_empty(), "COMMAND_SVC must not be empty");
}

#[test]
fn test_command_svc_constant_no_whitespace_edge() {
    assert!(
        !COMMAND_SVC.contains(char::is_whitespace),
        "COMMAND_SVC must not contain whitespace"
    );
}
