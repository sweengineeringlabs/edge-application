//! Layer-level coverage for the small request/response value types declared under
//! `api/clock/dto/` that have no dedicated per-type test file (SEA layer test
//! coverage, `sea_layer_test_coverage`). Each test constructs the type through the
//! crate's public API and asserts on its real shape or field values.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_application_clock::{ElapsedSinceEpochRequest, ElapsedSinceEpochResponse, NowRequest, NowResponse};

/// @covers: ElapsedSinceEpochRequest
#[test]
fn test_elapsed_since_epoch_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ElapsedSinceEpochRequest>(), 0);
    let _ = ElapsedSinceEpochRequest;
}

/// @covers: ElapsedSinceEpochResponse
#[test]
fn test_elapsed_since_epoch_response_holds_duration_happy() {
    let r = ElapsedSinceEpochResponse {
        duration: Duration::from_secs(5),
    };
    assert_eq!(r.duration, Duration::from_secs(5));
}

/// @covers: NowRequest
#[test]
fn test_now_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<NowRequest>(), 0);
    let _ = NowRequest;
}

/// @covers: NowResponse
#[test]
fn test_now_response_holds_instant_happy() {
    let r = NowResponse {
        instant: SystemTime::UNIX_EPOCH,
    };
    assert_eq!(r.instant, SystemTime::UNIX_EPOCH);
}
