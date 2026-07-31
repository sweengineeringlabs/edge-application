//! [`SpecMatchesResponse`] — wrapper for a specification match check.

/// Result of [`Spec::matches`](crate::api::repository::traits::Spec::matches).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecMatchesResponse {
    /// `true` if the entity satisfies the specification.
    pub matches: bool,
}
