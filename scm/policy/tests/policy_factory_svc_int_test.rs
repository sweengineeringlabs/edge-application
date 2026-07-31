//! SAF facade tests — `CompositePolicy` construction.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_policy::{CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse, PolicyError};

struct AlwaysFails;
impl Policy for AlwaysFails {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "always-fails" })
    }
    fn evaluate(&self, _req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        Err(PolicyError::new("always-fails", "denied"))
    }
}

/// @covers: CompositePolicy::new — empty composite always passes
#[test]
fn test_composite_empty_always_passes_happy() {
    let policy = CompositePolicy::<String>::new();
    assert_eq!(
        policy.evaluate(PolicyEvaluateRequest { input: &"anything".to_string() }),
        Ok(()),
        "empty composite should pass"
    );
}

/// @covers: CompositePolicy::new — first failing rule rejects input
#[test]
fn test_composite_with_failing_rule_rejects_input_error() {
    let policy = CompositePolicy::<String>::new().with(Box::new(AlwaysFails));
    let result = policy.evaluate(PolicyEvaluateRequest { input: &"input".to_string() });
    assert!(matches!(result, Err(v) if v.policy == "always-fails"), "should reject with always-fails policy");
}

/// @covers: CompositePolicy::new — generic over input type
#[test]
fn test_composite_generic_over_input_type_edge() {
    let policy = CompositePolicy::<u64>::new();
    assert_eq!(policy.evaluate(PolicyEvaluateRequest { input: &42 }), Ok(()), "empty composite with u64 should pass");
}
