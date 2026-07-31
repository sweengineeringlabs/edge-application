use edge_application_observer::ObserveError;

#[test]
fn test_observe_error_backend_unavailable_display_contains_message() {
    let err = ObserveError::BackendUnavailable("otel not reachable".to_string());
    let msg = err.to_string();
    assert!(msg.contains("otel not reachable"), "got: {msg}");
}

#[test]
fn test_observe_error_not_initialised_display_is_non_empty() {
    let err = ObserveError::NotInitialised;
    let msg = err.to_string();
    assert!(!msg.is_empty());
}

#[test]
fn test_observe_error_debug_does_not_panic() {
    let err = ObserveError::NotInitialised;
    let debug_output = format!("{err:?}");
    assert!(!debug_output.is_empty(), "debug output must not be empty");
}

#[test]
fn test_observe_error_eq_same_variant_equal() {
    let a = ObserveError::NotInitialised;
    let b = ObserveError::NotInitialised;
    assert_eq!(a, b);
}

#[test]
fn test_observe_error_eq_different_variants_not_equal() {
    let a = ObserveError::NotInitialised;
    let b = ObserveError::BackendUnavailable("x".to_string());
    assert_ne!(a, b);
}
