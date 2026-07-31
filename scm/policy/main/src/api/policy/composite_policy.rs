//! `CompositePolicy<I>` — AND-composition of multiple [`Policy`] rules.

use crate::api::policy::traits::Policy;

/// Combines multiple [`Policy<Input = I>`](crate::api::policy::traits::Policy)
/// rules with AND semantics.
///
/// Each policy is evaluated in registration order. The first
/// [`PolicyError`](crate::api::policy::errors::PolicyError) encountered
/// short-circuits evaluation and is returned. An empty composite always passes.
///
/// # Examples
///
/// ```rust
/// use edge_application_policy::{
///     CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse,
///     PolicyError,
/// };
///
/// struct MaxLength(usize);
///
/// impl Policy for MaxLength {
///     type Input = String;
///     fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
///         Ok(PolicyNameResponse { name: "max-length" })
///     }
///     fn evaluate(&self, req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
///         if req.input.len() <= self.0 { Ok(()) }
///         else { Err(PolicyError::new("max-length", format!("exceeds {}", self.0))) }
///     }
/// }
///
/// let policy = CompositePolicy::new()
///     .with(Box::new(MaxLength(10)));
///
/// assert!(policy.evaluate(PolicyEvaluateRequest { input: &"short".to_string() }).is_ok());
/// assert!(policy.evaluate(PolicyEvaluateRequest { input: &"a string that is too long".to_string() }).is_err());
/// ```
pub struct CompositePolicy<I> {
    pub(crate) name: &'static str,
    pub(crate) policies: Vec<Box<dyn Policy<Input = I>>>,
}
