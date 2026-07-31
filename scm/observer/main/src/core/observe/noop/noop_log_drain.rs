use crate::api::LogDrain;
use crate::api::LogEmitRequest;
use crate::api::LogEmitResponse;
use crate::api::NoopLogDrain;
use crate::api::ObserveError;

impl NoopLogDrain {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl LogDrain for NoopLogDrain {
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, ObserveError> {
        let _ = req;
        Ok(LogEmitResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_noop_log_drain_happy() {
        let d = NoopLogDrain::new();
        assert_eq!(std::mem::size_of_val(&d), 0);
    }

    #[test]
    fn test_emit_info_record_discarded_error() {
        let d = NoopLogDrain::new();
        d.emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
        assert_eq!(std::mem::size_of_val(&d), 0);
    }

    #[test]
    fn test_emit_multiple_records_no_accumulation_edge() {
        let d = NoopLogDrain::new();
        for i in 0..10 {
            d.emit(LogEmitRequest {
                level: "DEBUG".to_string(),
                handler_id: "h".to_string(),
                message: format!("msg {i}"),
            })
            .unwrap();
        }
        assert_eq!(std::mem::size_of_val(&d), 0);
    }
}
