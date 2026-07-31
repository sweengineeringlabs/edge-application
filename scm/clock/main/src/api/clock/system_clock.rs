//! `SystemClock` — wall-clock `Clock` implementation.

/// Delegates every `now()` call to `std::time::SystemTime::now`.
pub struct SystemClock;
