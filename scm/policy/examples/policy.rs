//! Basic `Policy` usage example.

use edge_application_policy::{
    CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse,
    PolicyError,
};

struct MaxLength(usize);

impl Policy for MaxLength {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "max-length" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        if req.input.len() <= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new("max-length", format!("exceeds {}", self.0)))
        }
    }
}

fn main() {
    let policy: CompositePolicy<String> = CompositePolicy::new()
        .with(Box::new(MaxLength(5)));
    println!("{:?}", policy.evaluate(PolicyEvaluateRequest { input: &"hi".to_string() }));
    println!("{:?}", policy.evaluate(PolicyEvaluateRequest { input: &"too long".to_string() }));
}
