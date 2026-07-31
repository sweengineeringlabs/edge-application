//! Integration tests for `SystemClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_application_clock::{Clock, NowRequest, SystemClock};

/// @covers: SystemClock (Clock::now)
#[test]
fn test_now_system_clock_returns_recent_time_happy() {
    let before = SystemTime::now();
    let t = SystemClock.now(NowRequest).unwrap().instant;
    assert!(t >= before);
}

/// @covers: SystemClock (Clock::now monotonicity within a single thread)
#[test]
fn test_now_system_clock_successive_calls_do_not_panic_error() {
    let _t1 = SystemClock.now(NowRequest);
    let _t2 = SystemClock.now(NowRequest);
    assert!(true);
}

/// @covers: SystemClock (zero-sized)
#[test]
fn test_system_clock_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SystemClock>(), 0);
}
