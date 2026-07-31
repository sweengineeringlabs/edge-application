//! Integration tests for `MemoryRepository` — covers the types/ file directly.
// @allow: no_mocks_in_integration — MemoryRepository is the production-shipped reference impl, not a test double

use edge_application_repository::{
    MemoryRepository, Repository, RepositoryIdRequest, RepositoryListRequest,
    RepositorySaveRequest,
};
use futures::executor::block_on;

/// @covers: MemoryRepository::new — creates an empty store
#[test]
fn test_new_creates_empty_store_happy() {
    let repo = MemoryRepository::<String, u32>::new();
    let items = block_on(repo.list(RepositoryListRequest))
        .map(|r| r.items)
        .unwrap_or_else(|_| vec!["x".into()]);
    assert!(items.is_empty());
}

/// @covers: MemoryRepository — find missing id returns None
#[test]
fn test_find_missing_id_returns_none_error() {
    let repo = MemoryRepository::<String, u32>::new();
    let result = block_on(repo.find(RepositoryIdRequest { id: &99 }))
        .map(|r| r.entity)
        .unwrap_or(Some("x".into()));
    assert!(result.is_none());
}

/// @covers: MemoryRepository — two instances are isolated
#[test]
fn test_two_instances_are_isolated_edge() {
    let a = MemoryRepository::<String, u32>::new();
    let b = MemoryRepository::<String, u32>::new();
    block_on(a.save(RepositorySaveRequest {
        id: 1,
        entity: "hello".into(),
    }))
    .unwrap_or_default();
    let in_b = block_on(b.find(RepositoryIdRequest { id: &1 }))
        .map(|r| r.entity)
        .unwrap_or(Some("x".into()));
    assert!(in_b.is_none());
}
