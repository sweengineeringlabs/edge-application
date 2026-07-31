//! [`SpecMatchesRequest`] — request to test an entity against a specification.

/// Request to check whether `entity` satisfies a specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecMatchesRequest<'a, T> {
    /// The entity to test.
    pub entity: &'a T,
}
