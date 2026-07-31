use edge_application_app::AppError;

#[test]
fn test_app_error_boot_failed_message_happy() {
    let err = AppError::BootFailed("timeout".into());
    assert_eq!(err.to_string(), "boot failed: timeout");
}

#[test]
fn test_app_error_creation_failed_message_error() {
    let err = AppError::CreationFailed("missing dep".into());
    assert_eq!(err.to_string(), "service creation failed: missing dep");
}

#[test]
fn test_app_error_debug_output_is_non_empty_edge() {
    let err = AppError::BootFailed("x".into());
    let debug = format!("{err:?}");
    assert!(!debug.is_empty());
    assert!(debug.contains("BootFailed"));
}
