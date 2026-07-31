//! External tests proving the saf/ anchor consts are reachable.

#[cfg(test)]
mod tests {
    /// @covers: APP_RUNTIME_SVC
    #[test]
    fn test_app_runtime_svc_anchor_is_accessible_happy() {
        assert_eq!(crate::saf::APP_RUNTIME_SVC, "app_runtime");
    }

    /// @covers: APP_SERVICE_PROVIDER_SVC
    #[test]
    fn test_app_service_provider_svc_anchor_is_accessible_error() {
        assert_eq!(crate::saf::APP_SERVICE_PROVIDER_SVC, "app_service_provider");
    }

    /// @covers: APPLICATION_SVC
    #[test]
    fn test_application_svc_anchor_is_accessible_edge() {
        assert_eq!(crate::saf::APPLICATION_SVC, "application");
    }
}
