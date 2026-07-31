//! Integration tests for [`EmptyRequest`].

use edge_application_base::{EmptyRequest, Request};

fn accept_request<T: Request>(req: T) -> T {
    req
}

/// @covers: EmptyRequest — is zero-sized and constructible
#[test]
fn test_empty_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<EmptyRequest>(), 0);
    let _ = EmptyRequest;
}

/// @covers: EmptyRequest — satisfies the Request bound and can cross a generic boundary
#[test]
fn test_empty_request_satisfies_request_bound_error() {
    let req = accept_request(EmptyRequest);
    assert_eq!(req, EmptyRequest);
}

/// @covers: EmptyRequest — independent of EmptyResponse; usable standalone with no pairing
#[test]
fn test_empty_request_usable_without_empty_response_edge() {
    struct RealResponse(u32);
    impl edge_application_base::Response for RealResponse {}

    fn accept_response<T: edge_application_base::Response>(resp: T) -> T {
        resp
    }

    let req = accept_request(EmptyRequest);
    let resp = accept_response(RealResponse(42));
    assert_eq!(req, EmptyRequest);
    assert_eq!(resp.0, 42);
}
