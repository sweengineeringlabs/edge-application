//! [`Policy`] impl for [`CompositePolicy`], plus its constructors.

use crate::api::PolicyError;
use crate::api::Policy;
use crate::api::{CompositePolicy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse};

#[expect(
    dead_code,
    reason = "SEA core/ structural anchor — CompositePolicy is generic, so the file needs a concrete type matching its name"
)]
pub(crate) struct DefaultCompositePolicy;

impl<I> CompositePolicy<I> {
    /// Construct an empty composite. An empty composite always passes.
    pub fn new() -> Self {
        Self {
            name: "composite",
            policies: Vec::new(),
        }
    }

    /// Append `policy` to the evaluation chain.
    pub fn with(mut self, policy: Box<dyn Policy<Input = I>>) -> Self {
        self.policies.push(policy);
        self
    }
}

impl<I> Default for CompositePolicy<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Send + Sync + 'static> Policy for CompositePolicy<I> {
    type Input = I;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: self.name })
    }

    fn evaluate(&self, req: PolicyEvaluateRequest<'_, I>) -> Result<(), PolicyError> {
        for policy in &self.policies {
            policy.evaluate(PolicyEvaluateRequest { input: req.input })?;
        }
        Ok(())
    }
}
