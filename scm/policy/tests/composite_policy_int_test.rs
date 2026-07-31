//! Integration tests for `CompositePolicy` AND-composition.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_policy::{CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse, PolicyError};

struct Reject(&'static str);
impl Policy for Reject {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: self.0 })
    }
    fn evaluate(&self, _req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        Err(PolicyError::new(self.0, "denied"))
    }
}

struct Accept;
impl Policy for Accept {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "accept" })
    }
    fn evaluate(&self, _req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        Ok(())
    }
}

fn eval(input: &String) -> PolicyEvaluateRequest<'_, String> {
    PolicyEvaluateRequest { input }
}

/// @covers: CompositePolicy::new + evaluate — empty composite passes
#[test]
fn test_evaluate_empty_composite_passes_happy() {
    let c: CompositePolicy<String> = CompositePolicy::new();
    assert_eq!(c.evaluate(eval(&"x".to_string())), Ok(()), "empty composite should pass");
}

/// @covers: CompositePolicy::with + evaluate — failing member short-circuits
#[test]
fn test_evaluate_failing_member_returns_err_error() {
    let c = CompositePolicy::new()
        .with(Box::new(Accept))
        .with(Box::new(Reject("second")));
    let err = c.evaluate(eval(&"x".to_string()));
    assert!(err.is_err());
}

/// @covers: CompositePolicy — first violation wins
#[test]
fn test_evaluate_first_violation_short_circuits_edge() {
    let c = CompositePolicy::new()
        .with(Box::new(Reject("first")))
        .with(Box::new(Reject("second")));
    let err = c.evaluate(eval(&"x".to_string())).unwrap_err();
    assert_eq!(err.policy, "first");
}
