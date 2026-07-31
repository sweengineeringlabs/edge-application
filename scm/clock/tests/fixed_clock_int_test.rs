//! Integration tests for `FixedClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_application_clock::{Clock, FixedClock, NowRequest};

/// @covers: FixedClock::new, Clock::now
#[test]
fn test_now_returns_exact_configured_instant_happy() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let clock = FixedClock::new(instant);
    assert_eq!(clock.now(NowRequest).unwrap().instant, instant);
}

/// @covers: FixedClock (Clock::now determinism)
#[test]
fn test_now_repeated_calls_return_same_instant_error() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(999);
    let clock = FixedClock::new(instant);
    let t1 = clock.now(NowRequest).unwrap().instant;
    let t2 = clock.now(NowRequest).unwrap().instant;
    // Verify monotonicity — time should never go backward
    assert!(t2 >= t1, "Clock should be monotonic");
    // And verify they're equal since this is a fixed clock
    assert_eq!(t1, instant);
    assert_eq!(t2, instant);
}

/// @covers: FixedClock (Clock::now at UNIX_EPOCH)
#[test]
fn test_now_at_unix_epoch_returns_epoch_edge() {
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH);
    assert_eq!(clock.now(NowRequest).unwrap().instant, SystemTime::UNIX_EPOCH);
}
