//! `PolicyError` — describes why a [`Policy`](crate::api::policy::traits::Policy) was violated.

use thiserror::Error;

/// Returned by [`Policy::evaluate`](crate::api::policy::traits::Policy::evaluate)
/// when a business rule is not satisfied.
///
/// # Examples
///
/// ```rust
/// use edge_application_policy::PolicyError;
///
/// let v = PolicyError::new("spending-limit", "transfer of 500 exceeds daily limit of 200");
/// assert_eq!(v.policy, "spending-limit");
/// assert!(v.reason.contains("500"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("policy '{policy}' violated: {reason}")]
pub struct PolicyError {
    /// The [`Policy::name`](crate::api::policy::traits::Policy::name) of the rule that failed.
    pub policy: &'static str,

    /// Human-readable explanation of why the rule was not satisfied.
    pub reason: String,
}
