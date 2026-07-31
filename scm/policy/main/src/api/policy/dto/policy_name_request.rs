//! [`PolicyNameRequest`] — zero-sized marker for querying a policy's name.

/// Request for a [`Policy`](crate::api::policy::traits::Policy)'s human-readable name.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PolicyNameRequest;
