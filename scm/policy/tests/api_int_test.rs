//! Layer-level coverage for `api/policy/dto/*.rs` request/response types.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_policy::{Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse};

/// @covers: PolicyNameRequest
#[test]
fn test_policy_name_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<PolicyNameRequest>(), 0);
    let _ = PolicyNameRequest;
}

/// @covers: PolicyNameResponse
#[test]
fn test_policy_name_response_holds_name_happy() {
    let r = PolicyNameResponse { name: "max-length" };
    assert_eq!(r.name, "max-length");
}

/// @covers: PolicyEvaluateRequest
#[test]
fn test_policy_evaluate_request_used_by_policy_impl_happy() {
    struct AlwaysOk;
    impl Policy for AlwaysOk {
        type Input = u64;
        fn name(
            &self,
            _req: PolicyNameRequest,
        ) -> Result<PolicyNameResponse, edge_application_policy::PolicyError> {
            Ok(PolicyNameResponse { name: "always-ok" })
        }
        fn evaluate(
            &self,
            _req: PolicyEvaluateRequest<'_, u64>,
        ) -> Result<(), edge_application_policy::PolicyError> {
            Ok(())
        }
    }
    let value = 5u64;
    let req = PolicyEvaluateRequest { input: &value };
    assert_eq!(req.input, &5);
    assert!(AlwaysOk.evaluate(PolicyEvaluateRequest { input: &value }).is_ok());
}
