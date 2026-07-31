#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{LogDrain, LogEmitRequest, StdObserveFactory, LOG_DRAIN_SVC};

#[test]
fn test_noop_log_drain_svc_emits_info_record_happy() {
    let drain = StdObserveFactory::noop_log_drain();
    drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "svc.x".to_string(),
            message: "started".to_string(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_noop_log_drain_svc_emits_unknown_level_error() {
    let drain = StdObserveFactory::noop_log_drain();
    drain
        .emit(LogEmitRequest {
            level: "UNKNOWN".to_string(),
            handler_id: "svc.x".to_string(),
            message: "unexpected level".to_string(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_noop_log_drain_svc_emits_empty_fields_edge() {
    let drain = StdObserveFactory::noop_log_drain();
    drain
        .emit(LogEmitRequest {
            level: String::new(),
            handler_id: String::new(),
            message: String::new(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_log_drain_svc_key_namespaced_happy() {
    assert!(LOG_DRAIN_SVC.starts_with("edge."));
}

#[test]
fn test_log_drain_svc_returns_dyn_trait_object() {
    let _drain: Box<dyn LogDrain> = StdObserveFactory::noop_log_drain();
}
