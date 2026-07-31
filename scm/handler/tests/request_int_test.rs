//! Integration tests — `edge_application_handler::Request` resolves to the shared
//! `edge_application_base::Request` contract every `Handler::Request`/`Service::Request`
//! associated type is bound against.

use edge_application_base::Request as BaseRequest;
use edge_application_handler::Request;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl BaseRequest for TextPayload {}

fn accept_request<T: Request>(req: T) -> T {
    req
}

/// @covers: Request — resolves to the same contract as edge_application_base::Request
#[test]
fn test_request_accepts_base_request_impl_happy() {
    let req = accept_request(TextPayload("hello".to_string()));
    assert_eq!(req.0, "hello");
}

/// @covers: Request — a zero-sized type implementing only the base contract still satisfies
/// the re-export (the bound carries no extra requirement beyond edge_application_base::Request)
#[test]
fn test_request_accepts_zero_sized_base_impl_edge() {
    struct Ping;
    impl BaseRequest for Ping {}

    let req = accept_request(Ping);
    assert_eq!(std::mem::size_of_val(&req), 0);
}
