//! Integration tests for `Spec` — predicate trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_repository::{RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse};

struct AlwaysTrue;
impl Spec for AlwaysTrue {
    type Entity = u32;

    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, u32>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: true })
    }
}

struct AlwaysFalse;
impl Spec for AlwaysFalse {
    type Entity = u32;
}

struct GreaterThan(u32);
impl Spec for GreaterThan {
    type Entity = u32;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, u32>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: *req.entity > self.0,
        })
    }
}

/// @covers: Spec::matches — custom impl returning true
#[test]
fn test_matches_always_true_spec_returns_true_happy() {
    let spec = AlwaysTrue;
    assert!(
        spec.matches(SpecMatchesRequest { entity: &0 })
            .unwrap()
            .matches
    );
    assert!(
        spec.matches(SpecMatchesRequest { entity: &u32::MAX })
            .unwrap()
            .matches
    );
}

/// @covers: Spec::matches — default impl returns false
#[test]
fn test_matches_default_impl_returns_false_error() {
    let spec = AlwaysFalse;
    assert!(
        !spec
            .matches(SpecMatchesRequest { entity: &42 })
            .unwrap()
            .matches
    );
}

/// @covers: Spec::matches — boundary value at threshold
#[test]
fn test_matches_boundary_value_at_threshold_edge() {
    let spec = GreaterThan(10);
    assert!(
        !spec
            .matches(SpecMatchesRequest { entity: &10 })
            .unwrap()
            .matches
    );
    assert!(
        spec.matches(SpecMatchesRequest { entity: &11 })
            .unwrap()
            .matches
    );
}
