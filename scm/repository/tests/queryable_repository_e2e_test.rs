//! SAF facade tests — `QueryableRepository` trait via `MemoryRepository`.
// @allow: no_mocks_in_integration — MemoryRepository is the production-shipped reference impl, not a test double

use edge_application_repository::{
    MemoryRepository, QueryableRepository, Repository, RepositoryError, RepositorySaveRequest,
    Spec, SpecMatchesRequest, SpecMatchesResponse, SpecRequest,
};
use futures::executor::block_on;

fn make() -> MemoryRepository<String, u32> {
    MemoryRepository::new()
}

struct StartsWithA;
impl Spec for StartsWithA {
    type Entity = String;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, String>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: req.entity.starts_with('a'),
        })
    }
}

struct NeverMatches;
impl Spec for NeverMatches {
    type Entity = String;
}

/// @covers: QueryableRepository::find_by — returns all matching entities
#[test]
fn test_find_by_matching_spec_returns_all_matches_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "alpha".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 2,
        entity: "beta".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 3,
        entity: "ant".into(),
    }))
    .unwrap_or_default();
    let results = block_on(repo.find_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.items)
    .unwrap_or_default();
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|s| s.starts_with('a')));
}

/// @covers: QueryableRepository::find_by — no match returns empty vec
#[test]
fn test_find_by_no_match_returns_empty_error() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "beta".into(),
    }))
    .unwrap_or_default();
    let results = block_on(repo.find_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.items)
    .unwrap_or_else(|_| vec!["x".into()]);
    assert!(results.is_empty());
}

/// @covers: QueryableRepository::find_by — empty repo returns empty vec
#[test]
fn test_find_by_empty_repo_returns_empty_edge() {
    let repo = make();
    let results = block_on(repo.find_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.items)
    .unwrap_or_else(|_| vec!["x".into()]);
    assert!(results.is_empty());
}

/// @covers: QueryableRepository::find_one_by — returns first matching entity
#[test]
fn test_find_one_by_matching_spec_returns_first_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "alpha".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 2,
        entity: "beta".into(),
    }))
    .unwrap_or_default();
    let found = block_on(repo.find_one_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.entity)
    .unwrap_or(None);
    assert!(found
        .as_deref()
        .map(|s: &str| s.starts_with('a'))
        .unwrap_or(false));
}

/// @covers: QueryableRepository::find_one_by — no match returns None
#[test]
fn test_find_one_by_no_match_returns_none_error() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "beta".into(),
    }))
    .unwrap_or_default();
    let found = block_on(repo.find_one_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.entity)
    .unwrap_or(Some("x".into()));
    assert!(found.is_none());
}

/// @covers: QueryableRepository::find_one_by — empty repo returns None
#[test]
fn test_find_one_by_empty_repo_returns_none_edge() {
    let repo = make();
    let found = block_on(repo.find_one_by(SpecRequest {
        spec: Box::new(NeverMatches),
    }))
    .map(|r| r.entity)
    .unwrap_or(Some("x".into()));
    assert!(found.is_none());
}

/// @covers: QueryableRepository::count_by — returns correct count
#[test]
fn test_count_by_returns_correct_count_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "ant".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 2,
        entity: "bear".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 3,
        entity: "ape".into(),
    }))
    .unwrap_or_default();
    let n = block_on(repo.count_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.count)
    .unwrap_or(0);
    assert_eq!(n, 2);
}

/// @covers: QueryableRepository::count_by — no match returns zero
#[test]
fn test_count_by_no_match_returns_zero_error() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "beta".into(),
    }))
    .unwrap_or_default();
    let n = block_on(repo.count_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.count)
    .unwrap_or(1);
    assert_eq!(n, 0);
}

/// @covers: QueryableRepository::count_by — empty repo returns zero
#[test]
fn test_count_by_empty_repo_returns_zero_edge() {
    let repo = make();
    let n = block_on(repo.count_by(SpecRequest {
        spec: Box::new(StartsWithA),
    }))
    .map(|r| r.count)
    .unwrap_or(1);
    assert_eq!(n, 0);
}
