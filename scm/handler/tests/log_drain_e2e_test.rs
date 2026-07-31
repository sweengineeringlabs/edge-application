//! SAF facade tests — `LogDrain` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Mutex;

use edge_application_handler::{LogDrain, LogEmitRequest, LogEmitResponse, ObserveError};

#[derive(Default)]
struct RecordingLogDrain {
    records: Mutex<Vec<LogEmitRequest>>,
}
impl LogDrain for RecordingLogDrain {
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, ObserveError> {
        self.records.lock().unwrap().push(req);
        Ok(LogEmitResponse)
    }
}

/// @covers: LogDrain::emit — record fields are preserved
#[test]
fn test_emit_info_record_is_stored_happy() {
    let drain = RecordingLogDrain::default();
    drain
        .emit(LogEmitRequest {
            level: "INFO".into(),
            handler_id: "order.create".into(),
            message: "created".into(),
        })
        .expect("emit should succeed");
    let records = drain.records.lock().unwrap();
    assert_eq!(records[0].level, "INFO");
    assert_eq!(records[0].handler_id, "order.create");
    assert_eq!(records[0].message, "created");
}

/// @covers: LogDrain::emit — empty message is still emitted, not rejected
#[test]
fn test_emit_empty_message_still_recorded_error() {
    let drain = RecordingLogDrain::default();
    drain
        .emit(LogEmitRequest {
            level: "ERROR".into(),
            handler_id: "h".into(),
            message: String::new(),
        })
        .expect("emit should succeed");
    assert_eq!(drain.records.lock().unwrap()[0].message, "");
}

/// @covers: LogDrain::emit — multiple records accumulate in order
#[test]
fn test_emit_multiple_records_accumulate_in_order_edge() {
    let drain = RecordingLogDrain::default();
    drain
        .emit(LogEmitRequest {
            level: "INFO".into(),
            handler_id: "h".into(),
            message: "first".into(),
        })
        .unwrap();
    drain
        .emit(LogEmitRequest {
            level: "WARN".into(),
            handler_id: "h".into(),
            message: "second".into(),
        })
        .unwrap();
    let records = drain.records.lock().unwrap();
    assert_eq!(records.len(), 2);
    assert_eq!(records[1].message, "second");
}
