//! `impl PolicyError` — constructor.

use crate::api::PolicyError;

impl PolicyError {
    /// Construct a `PolicyError` with the given policy name and reason.
    pub fn new(policy: &'static str, reason: impl Into<String>) -> Self {
        Self {
            policy,
            reason: reason.into(),
        }
    }
}
