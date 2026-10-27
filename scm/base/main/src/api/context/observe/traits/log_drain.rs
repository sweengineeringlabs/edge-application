//! `LogDrain` — structured log emission contract.

use crate::api::context::observe::dto::{LogEmitRequest, LogEmitResponse};
use crate::api::context::observe::errors::ObserveError;

/// Receives structured log records emitted by domain handlers.
pub trait LogDrain: Send + Sync {
    /// Emit a structured log record.
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, ObserveError>;
}
