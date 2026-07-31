//! `Clock` — injectable time source.

use std::time::SystemTime;

use crate::api::clock::errors::ClockError;
use crate::api::clock::dto::{
    ElapsedSinceEpochRequest, ElapsedSinceEpochResponse, NowRequest, NowResponse,
};

/// Source of the current wall time.
///
/// Inject `SystemClock` in production and `FixedClock` in tests to make
/// time-dependent domain logic deterministic.
pub trait Clock: Send + Sync {
    /// Return the current instant.
    fn now(&self, req: NowRequest) -> Result<NowResponse, ClockError>;

    /// Return the elapsed duration since the Unix epoch.
    ///
    /// Returns [`ClockError::BeforeEpoch`] when the clock reports a time
    /// earlier than the Unix epoch.
    fn elapsed_since_epoch(
        &self,
        _req: ElapsedSinceEpochRequest,
    ) -> Result<ElapsedSinceEpochResponse, ClockError> {
        let instant = self.now(NowRequest)?.instant;
        let duration = instant
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| ClockError::BeforeEpoch)?;
        Ok(ElapsedSinceEpochResponse { duration })
    }
}
