//! `Spec` — predicate type for filtering entities in a repository query.

use crate::api::repository::errors::RepositoryError;
use crate::api::repository::dto::{SpecMatchesRequest, SpecMatchesResponse};

/// A specification predicate used to filter entities of type [`Entity`](Spec::Entity).
///
/// Implement this trait to express domain query criteria without exposing
/// persistence details.
pub trait Spec: Send + Sync {
    /// The entity type this specification filters.
    type Entity: Send + Sync;

    /// Returns `true` if the requested entity satisfies this specification.
    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, Self::Entity>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: false })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysMatch;
    impl Spec for AlwaysMatch {
        type Entity = u32;

        fn matches(
            &self,
            _req: SpecMatchesRequest<'_, u32>,
        ) -> Result<SpecMatchesResponse, RepositoryError> {
            Ok(SpecMatchesResponse { matches: true })
        }
    }

    struct NeverMatch;
    impl Spec for NeverMatch {
        type Entity = u32;
    }

    #[test]
    fn test_matches_custom_impl_returns_true_happy() {
        let spec = AlwaysMatch;
        let entity = 42u32;
        assert!(
            spec.matches(SpecMatchesRequest { entity: &entity })
                .unwrap()
                .matches
        );
    }

    #[test]
    fn test_matches_default_impl_returns_false_error() {
        let spec = NeverMatch;
        let entity = 42u32;
        assert!(
            !spec
                .matches(SpecMatchesRequest { entity: &entity })
                .unwrap()
                .matches
        );
    }

    #[test]
    fn test_matches_default_impl_consistent_across_values_edge() {
        let spec = NeverMatch;
        let a = 0u32;
        let b = u32::MAX;
        assert!(
            !spec
                .matches(SpecMatchesRequest { entity: &a })
                .unwrap()
                .matches
        );
        assert!(
            !spec
                .matches(SpecMatchesRequest { entity: &b })
                .unwrap()
                .matches
        );
    }
}
