//! SAF facade tests — `Policy` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_policy::{Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyError};

struct NonZero;
impl Policy for NonZero {
    type Input = u64;
    fn name(&self, _req: PolicyNameRequest) -> Result<edge_application_policy::PolicyNameResponse, PolicyError> {
        Ok(edge_application_policy::PolicyNameResponse { name: "non-zero" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input > 0 {
            Ok(())
        } else {
            Err(PolicyError::new("non-zero", "must be > 0"))
        }
    }
}

fn eval(input: &u64) -> PolicyEvaluateRequest<'_, u64> {
    PolicyEvaluateRequest { input }
}

/// @covers: Policy::name
#[test]
fn test_name_returns_static_label_happy() {
    assert_eq!(NonZero.name(PolicyNameRequest).unwrap().name, "non-zero");
}

/// @covers: Policy::name — stable across instances
#[test]
fn test_name_is_stable_across_calls_error() {
    let p = NonZero;
    let name1 = p.name(PolicyNameRequest).unwrap().name;
    let name2 = p.name(PolicyNameRequest).unwrap().name;
    assert_eq!(name1, name2, "Policy name must be stable across calls");
    assert_eq!(name1, "non-zero", "Policy name must match expected value");
}

/// @covers: Policy::name — usable via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_edge() {
    let p: &dyn Policy<Input = u64> = &NonZero;
    assert_eq!(p.name(PolicyNameRequest).unwrap().name, "non-zero");
}

/// @covers: Policy::evaluate — satisfied rule returns Ok
#[test]
fn test_evaluate_satisfied_rule_returns_ok_happy() {
    assert_eq!(NonZero.evaluate(eval(&5)), Ok(()));
}

/// @covers: Policy::evaluate — violated rule returns Err
#[test]
fn test_evaluate_violated_rule_returns_err_error() {
    let err = NonZero.evaluate(eval(&0));
    assert!(err.is_err());
}

/// @covers: Policy::evaluate — boundary at zero
#[test]
fn test_evaluate_boundary_value_edge() {
    assert!(NonZero.evaluate(eval(&1)).is_ok());
    assert!(NonZero.evaluate(eval(&0)).is_err());
}
