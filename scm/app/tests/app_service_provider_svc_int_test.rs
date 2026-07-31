//! Integration tests — `APP_SERVICE_PROVIDER_SVC` constant.

use edge_application_app::APP_SERVICE_PROVIDER_SVC;

/// @covers: APP_SERVICE_PROVIDER_SVC — correct service identity value
#[test]
fn test_app_service_provider_svc_constant_value_happy() {
    assert_eq!(APP_SERVICE_PROVIDER_SVC, "app_service_provider");
}

/// @covers: APP_SERVICE_PROVIDER_SVC — constant is non-empty
#[test]
fn test_app_service_provider_svc_constant_not_empty_error() {
    assert!(!APP_SERVICE_PROVIDER_SVC.is_empty());
    assert_eq!(APP_SERVICE_PROVIDER_SVC.len(), "app_service_provider".len());
}

/// @covers: APP_SERVICE_PROVIDER_SVC — constant contains no whitespace
#[test]
fn test_app_service_provider_svc_constant_no_whitespace_edge() {
    assert!(!APP_SERVICE_PROVIDER_SVC.contains(' '));
    assert!(!APP_SERVICE_PROVIDER_SVC.contains('\t'));
    assert_eq!(APP_SERVICE_PROVIDER_SVC, APP_SERVICE_PROVIDER_SVC.trim());
}
