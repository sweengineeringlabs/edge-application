//! Integration tests — `Clock` trait via `SystemClock`/`FixedClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_application_clock::{Clock, NowRequest, SystemClock};

/// @covers: Clock::now — system clock reports a recent time
#[test]
fn test_now_system_clock_reports_recent_time_happy() {
    let before = SystemTime::now();
    let t = SystemClock.now(NowRequest).unwrap().instant;
    assert!(t >= before);
}

/// @covers: Clock::now — repeated calls do not regress
#[test]
fn test_now_repeated_calls_do_not_regress_error() {
    let t1 = SystemClock.now(NowRequest).unwrap().instant;
    let t2 = SystemClock.now(NowRequest).unwrap().instant;
    assert!(t2 >= t1);
}
