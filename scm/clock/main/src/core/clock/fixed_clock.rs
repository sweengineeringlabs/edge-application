//! `Clock` impl for `FixedClock`.

use crate::api::Clock;
use crate::api::ClockError;
use crate::api::FixedClock;
use crate::api::NowRequest;
use crate::api::NowResponse;

impl FixedClock {
    /// Construct a clock frozen at `instant`.
    pub fn new(instant: std::time::SystemTime) -> Self {
        Self { instant }
    }
}

impl Clock for FixedClock {
    fn now(&self, _req: NowRequest) -> Result<NowResponse, ClockError> {
        Ok(NowResponse {
            instant: self.instant,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::time::SystemTime;

    #[test]
    fn test_now_returns_pinned_instant() {
        let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(123);
        assert_eq!(
            FixedClock::new(instant).now(NowRequest).unwrap().instant,
            instant
        );
    }
}
