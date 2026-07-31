//! Integration tests — `APP_RUNTIME_SVC_FACTORY` constant.

use edge_application_app::APP_RUNTIME_SVC_FACTORY;

/// @covers: APP_RUNTIME_SVC_FACTORY — correct factory identity value
#[test]
fn test_app_runtime_svc_factory_constant_value_happy() {
    assert_eq!(APP_RUNTIME_SVC_FACTORY, "app_runtime_factory");
}

/// @covers: APP_RUNTIME_SVC_FACTORY — constant is non-empty
#[test]
fn test_app_runtime_svc_factory_constant_not_empty_error() {
    assert!(!APP_RUNTIME_SVC_FACTORY.is_empty());
    assert_eq!(APP_RUNTIME_SVC_FACTORY.len(), "app_runtime_factory".len());
}

/// @covers: APP_RUNTIME_SVC_FACTORY — constant contains no whitespace
#[test]
fn test_app_runtime_svc_factory_constant_no_whitespace_edge() {
    assert!(!APP_RUNTIME_SVC_FACTORY.contains(' '));
    assert!(!APP_RUNTIME_SVC_FACTORY.contains('\t'));
    assert_eq!(APP_RUNTIME_SVC_FACTORY, APP_RUNTIME_SVC_FACTORY.trim());
}
