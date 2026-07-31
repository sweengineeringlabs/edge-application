//! Integration tests — `APP_RUNTIME_SVC` constant.

use edge_application_app::APP_RUNTIME_SVC;

/// @covers: APP_RUNTIME_SVC — correct service identity value
#[test]
fn test_app_runtime_svc_constant_value_happy() {
    assert_eq!(APP_RUNTIME_SVC, "app_runtime");
}

/// @covers: APP_RUNTIME_SVC — constant is non-empty
#[test]
fn test_app_runtime_svc_constant_not_empty_error() {
    assert!(!APP_RUNTIME_SVC.is_empty());
    assert_eq!(APP_RUNTIME_SVC.len(), "app_runtime".len());
}

/// @covers: APP_RUNTIME_SVC — constant contains no whitespace
#[test]
fn test_app_runtime_svc_constant_no_whitespace_edge() {
    assert!(!APP_RUNTIME_SVC.contains(' '));
    assert!(!APP_RUNTIME_SVC.contains('\t'));
    assert_eq!(APP_RUNTIME_SVC, APP_RUNTIME_SVC.trim());
}
