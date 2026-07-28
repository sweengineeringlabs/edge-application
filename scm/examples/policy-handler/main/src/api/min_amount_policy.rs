//! `MinAmountPolicy` — rejects a withdrawal amount below a fixed minimum.

use edge_application_policy::{Policy, PolicyEvaluateRequest, PolicyError, PolicyNameRequest, PolicyNameResponse};

pub struct MinAmountPolicy(pub u64);

impl Policy for MinAmountPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "min-amount" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input >= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "min-amount",
                format!("{} is below the minimum of {}", req.input, self.0),
            ))
        }
    }
}
