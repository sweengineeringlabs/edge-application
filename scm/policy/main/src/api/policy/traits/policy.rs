//! `Policy` — a named, testable business rule.

use crate::api::policy::errors::PolicyError;
use crate::api::policy::dto::{PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse};

/// A named, testable business rule that operates on domain state.
///
/// Distinct from `Validator`, which checks structural correctness — a policy
/// evaluates business intent against current domain state.
///
/// Use [`CompositePolicy`](crate::api::policy::composite_policy::CompositePolicy)
/// to combine multiple policies with AND semantics.
pub trait Policy: Send + Sync {
    /// The input type this policy evaluates.
    type Input;

    /// A human-readable name for this policy.
    fn name(&self, req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError>;

    /// Evaluate the policy against the request's input.
    ///
    /// Returns `Ok(())` if the rule is satisfied, or `Err(PolicyError)`.
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, Self::Input>) -> Result<(), PolicyError>;
}
