//! `Clock` impl for `SystemClock`.

use std::time::SystemTime;

use crate::api::Clock;
use crate::api::ClockError;
use crate::api::NowRequest;
use crate::api::NowResponse;
use crate::api::SystemClock;

impl Clock for SystemClock {
    fn now(&self, _req: NowRequest) -> Result<NowResponse, ClockError> {
        Ok(NowResponse {
            instant: SystemTime::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_advances_from_epoch() {
        assert!(SystemClock.now(NowRequest).unwrap().instant > SystemTime::UNIX_EPOCH);
    }
}
