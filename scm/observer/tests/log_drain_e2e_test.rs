#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{LogDrain, LogEmitRequest, StdObserveFactory};

// --- emit ---

#[test]
fn test_emit_info_record_happy() {
    let drain = StdObserveFactory::noop_log_drain();
    let req = LogEmitRequest {
        level: "INFO".to_string(),
        handler_id: "handler_a".to_string(),
        message: "request processed".to_string(),
    };
    drain.emit(req).unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_emit_error_level_record_error() {
    let drain = StdObserveFactory::noop_log_drain();
    let req = LogEmitRequest {
        level: "ERROR".to_string(),
        handler_id: "handler_c".to_string(),
        message: "unexpected failure".to_string(),
    };
    drain.emit(req).unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_emit_empty_fields_edge() {
    let drain = StdObserveFactory::noop_log_drain();
    let req = LogEmitRequest {
        level: String::new(),
        handler_id: String::new(),
        message: String::new(),
    };
    drain.emit(req).unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_log_drain_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let drain = StdObserveFactory::noop_log_drain();
    assert_send_sync(&drain);
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}

#[test]
fn test_log_drain_returns_dyn_trait_object() {
    let drain: Box<dyn LogDrain> = StdObserveFactory::noop_log_drain();
    drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*drain), 0, "noop log drain is ZST");
}
