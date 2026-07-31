//! SAF facade tests — `Spec` trait via direct implementation.
// @allow: no_mocks_in_integration — Spec is tested through concrete impls, not test doubles
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_repository::{RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse};

struct ExactMatch(u32);
impl Spec for ExactMatch {
    type Entity = u32;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, u32>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: *req.entity == self.0,
        })
    }
}

struct NeverMatch;
impl Spec for NeverMatch {
    type Entity = u32;
}

/// @covers: Spec::matches — exact match returns true
#[test]
fn test_matches_exact_value_returns_true_happy() {
    let spec = ExactMatch(42);
    assert!(
        spec.matches(SpecMatchesRequest { entity: &42 })
            .unwrap()
            .matches
    );
}

/// @covers: Spec::matches — non-matching value returns false
#[test]
fn test_matches_non_matching_value_returns_false_error() {
    let spec = ExactMatch(42);
    assert!(
        !spec
            .matches(SpecMatchesRequest { entity: &99 })
            .unwrap()
            .matches
    );
}

/// @covers: Spec::matches — default impl returns false for any value
#[test]
fn test_matches_default_impl_returns_false_for_zero_edge() {
    let spec = NeverMatch;
    assert!(
        !spec
            .matches(SpecMatchesRequest { entity: &0 })
            .unwrap()
            .matches
    );
}
