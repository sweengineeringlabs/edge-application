//! `FixedClock` — deterministic `Clock` for tests.

use std::time::SystemTime;

/// Returns a fixed `SystemTime` on every `now()` call.
pub struct FixedClock {
    pub(crate) instant: SystemTime,
}
