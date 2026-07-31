//! Integration tests — `edge_application_handler::Response` resolves to the shared
//! `edge_application_base::Response` contract every `Handler::Response`/`Service::Response`
//! associated type is bound against.

use edge_application_base::Response as BaseResponse;
use edge_application_handler::Response;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl BaseResponse for TextPayload {}

fn accept_response<T: Response>(resp: T) -> T {
    resp
}

/// @covers: Response — resolves to the same contract as edge_application_base::Response
#[test]
fn test_response_accepts_base_response_impl_happy() {
    let resp = accept_response(TextPayload("hello".to_string()));
    assert_eq!(resp.0, "hello");
}

/// @covers: Response — a zero-sized type implementing only the base contract still satisfies
/// the re-export (the bound carries no extra requirement beyond edge_application_base::Response)
#[test]
fn test_response_accepts_zero_sized_base_impl_edge() {
    struct Ping;
    impl BaseResponse for Ping {}

    let resp = accept_response(Ping);
    assert_eq!(std::mem::size_of_val(&resp), 0);
}
