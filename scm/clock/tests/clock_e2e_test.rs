//! SAF facade tests — `Clock` trait via `SystemClock` and `FixedClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_application_clock::{Clock, ElapsedSinceEpochRequest, FixedClock, NowRequest, SystemClock};

/// @covers: Clock::now (SystemClock)
#[test]
fn test_now_system_clock_returns_time_within_current_window_happy() {
    let before = SystemTime::now();
    let t = SystemClock.now(NowRequest).unwrap().instant;
    // SystemTime is not monotonic on Windows; assert forward progress only.
    assert!(t >= before);
}

/// @covers: Clock::now (SystemClock monotonicity)
#[test]
fn test_now_system_clock_successive_calls_do_not_go_backwards_error() {
    let t1 = SystemClock.now(NowRequest).unwrap().instant;
    let t2 = SystemClock.now(NowRequest).unwrap().instant;
    assert!(t2 >= t1);
}

/// @covers: Clock::now (FixedClock determinism)
#[test]
fn test_now_fixed_clock_repeated_calls_return_same_instant_edge() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
    let clock = FixedClock::new(instant);
    let t1 = clock.now(NowRequest).unwrap().instant;
    let t2 = clock.now(NowRequest).unwrap().instant;
    assert_eq!(t1, t2, "FixedClock must return same instant on repeated calls");
    assert_eq!(t1, instant, "FixedClock must return expected instant");
}

/// @covers: Clock::elapsed_since_epoch — post-epoch time yields a duration
#[test]
fn test_elapsed_since_epoch_post_epoch_returns_duration_happy() {
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH + Duration::from_secs(100));
    assert_eq!(
        clock.elapsed_since_epoch(ElapsedSinceEpochRequest).unwrap().duration,
        Duration::from_secs(100)
    );
}

/// @covers: Clock::elapsed_since_epoch — pre-epoch time errors
#[test]
fn test_elapsed_since_epoch_pre_epoch_returns_err_error() {
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH - Duration::from_secs(1));
    assert!(clock.elapsed_since_epoch(ElapsedSinceEpochRequest).is_err());
}

/// @covers: Clock::elapsed_since_epoch — exactly epoch is zero
#[test]
fn test_elapsed_since_epoch_at_epoch_is_zero_edge() {
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH);
    assert_eq!(
        clock.elapsed_since_epoch(ElapsedSinceEpochRequest).unwrap().duration,
        Duration::ZERO
    );
}
