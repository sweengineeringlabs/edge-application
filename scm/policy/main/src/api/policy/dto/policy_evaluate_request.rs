//! [`PolicyEvaluateRequest`] — request to evaluate a policy against an input.
// @allow: dto_types_must_serialize — holds a borrowed `&'a I` reference to the
// evaluation input, not owned wire-format data; a derived Deserialize cannot
// produce a borrowed reference with an unbounded lifetime.

/// Request to evaluate a [`Policy`](crate::api::policy::traits::Policy) against `input`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyEvaluateRequest<'a, I> {
    /// The value to evaluate the policy against.
    pub input: &'a I,
}
