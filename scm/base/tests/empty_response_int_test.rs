//! Integration tests for [`EmptyResponse`].

use edge_application_base::{EmptyResponse, Response};

fn accept_response<T: Response>(resp: T) -> T {
    resp
}

/// @covers: EmptyResponse — is zero-sized and constructible
#[test]
fn test_empty_response_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EmptyResponse>(), 0);
    let _ = EmptyResponse;
}

/// @covers: EmptyResponse — satisfies the Response bound and can cross a generic boundary
#[test]
fn test_empty_response_satisfies_response_bound_error() {
    let resp = accept_response(EmptyResponse);
    assert_eq!(resp, EmptyResponse);
}

/// @covers: EmptyResponse — independent of EmptyRequest; usable standalone with no pairing
#[test]
fn test_empty_response_usable_without_empty_request_edge() {
    struct RealRequest(u32);
    impl edge_application_base::Request for RealRequest {}

    fn accept_request<T: edge_application_base::Request>(req: T) -> T {
        req
    }

    let resp = accept_response(EmptyResponse);
    let req = accept_request(RealRequest(42));
    assert_eq!(resp, EmptyResponse);
    assert_eq!(req.0, 42);
}
