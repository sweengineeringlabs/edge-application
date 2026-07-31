//! `MaxAmountPolicy` — rejects a withdrawal amount above a fixed maximum.

use edge_application_policy::{Policy, PolicyEvaluateRequest, PolicyError, PolicyNameRequest, PolicyNameResponse};

pub struct MaxAmountPolicy(pub u64);

impl Policy for MaxAmountPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "max-amount" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input <= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "max-amount",
                format!("{} exceeds the maximum of {}", req.input, self.0),
            ))
        }
    }
}
