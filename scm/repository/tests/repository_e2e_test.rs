//! SAF facade tests — `Repository` trait via `MemoryRepository`.
// @allow: no_mocks_in_integration — MemoryRepository is the production-shipped reference impl, not a test double

use edge_application_repository::{
    MemoryRepository, Repository, RepositoryIdRequest, RepositoryListPageRequest,
    RepositoryListRequest, RepositorySaveRequest,
};
use futures::executor::block_on;

fn make() -> MemoryRepository<String, u32> {
    MemoryRepository::new()
}

/// @covers: Repository::save + find — round-trip
#[test]
fn test_save_then_find_returns_entity_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "order-1".into(),
    }))
    .unwrap_or_default();
    let found = block_on(repo.find(RepositoryIdRequest { id: &1 }))
        .map(|r| r.entity)
        .unwrap_or(None);
    assert_eq!(found.as_deref(), Some("order-1"));
}

/// @covers: Repository::find — missing id returns None
#[test]
fn test_find_missing_id_returns_none_error() {
    let repo = make();
    let found = block_on(repo.find(RepositoryIdRequest { id: &99 }))
        .map(|r| r.entity)
        .unwrap_or(Some("x".into()));
    assert!(found.is_none());
}

/// @covers: Repository::save — saving with an existing id does not corrupt other entries
#[test]
fn test_save_existing_id_leaves_other_entries_intact_error() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "original".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 2,
        entity: "other".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "updated".into(),
    }))
    .unwrap_or_default();
    let other = block_on(repo.find(RepositoryIdRequest { id: &2 }))
        .map(|r| r.entity)
        .unwrap_or(None);
    assert_eq!(other.as_deref(), Some("other"));
}

/// @covers: Repository::save — overwrites existing entry
#[test]
fn test_save_overwrites_existing_entry_edge() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "first".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "second".into(),
    }))
    .unwrap_or_default();
    let found = block_on(repo.find(RepositoryIdRequest { id: &1 }))
        .map(|r| r.entity)
        .unwrap_or(None);
    assert_eq!(found.as_deref(), Some("second"));
}

/// @covers: Repository::delete — removes existing entity, returns true
#[test]
fn test_delete_existing_entity_returns_true_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 2,
        entity: "item".into(),
    }))
    .unwrap_or_default();
    let removed = block_on(repo.delete(RepositoryIdRequest { id: &2 }))
        .map(|r| r.removed)
        .unwrap_or(false);
    assert!(removed);
    let found = block_on(repo.find(RepositoryIdRequest { id: &2 }))
        .map(|r| r.entity)
        .unwrap_or(Some("x".into()));
    assert!(found.is_none());
}

/// @covers: Repository::delete — missing entity returns false
#[test]
fn test_delete_missing_entity_returns_false_error() {
    let repo = make();
    let removed = block_on(repo.delete(RepositoryIdRequest { id: &42 }))
        .map(|r| r.removed)
        .unwrap_or(true);
    assert!(!removed);
}

/// @covers: Repository::delete — deleting the same id twice returns false the second time
#[test]
fn test_delete_already_deleted_returns_false_edge() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 3,
        entity: "item".into(),
    }))
    .unwrap_or_default();
    block_on(repo.delete(RepositoryIdRequest { id: &3 }))
        .unwrap_or(edge_application_repository::RepositoryDeleteResponse { removed: false });
    let second = block_on(repo.delete(RepositoryIdRequest { id: &3 }))
        .map(|r| r.removed)
        .unwrap_or(true);
    assert!(!second);
}

/// @covers: Repository::list — empty repo returns empty vec
#[test]
fn test_list_empty_repo_returns_empty_vec_edge() {
    let repo = make();
    let items = block_on(repo.list(RepositoryListRequest))
        .map(|r| r.items)
        .unwrap_or_else(|_| vec!["x".into()]);
    assert!(items.is_empty());
}

/// @covers: Repository::exists — saved entity returns true
#[test]
fn test_exists_saved_entity_returns_true_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 3,
        entity: "y".into(),
    }))
    .unwrap_or_default();
    let exists = block_on(repo.exists(RepositoryIdRequest { id: &3 }))
        .map(|r| r.exists)
        .unwrap_or(false);
    assert!(exists);
}

/// @covers: Repository::exists — missing entity returns false
#[test]
fn test_exists_missing_entity_returns_false_error() {
    let repo = make();
    let exists = block_on(repo.exists(RepositoryIdRequest { id: &7 }))
        .map(|r| r.exists)
        .unwrap_or(true);
    assert!(!exists);
}

/// @covers: Repository::exists — deleted entity no longer exists
#[test]
fn test_exists_deleted_entity_returns_false_edge() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 5,
        entity: "gone".into(),
    }))
    .unwrap_or_default();
    block_on(repo.delete(RepositoryIdRequest { id: &5 }))
        .unwrap_or(edge_application_repository::RepositoryDeleteResponse { removed: false });
    let exists = block_on(repo.exists(RepositoryIdRequest { id: &5 }))
        .map(|r| r.exists)
        .unwrap_or(true);
    assert!(!exists);
}

/// @covers: Repository::count — empty repo returns zero
#[test]
fn test_count_empty_repo_returns_zero_edge() {
    let repo = make();
    let n = block_on(repo.count(RepositoryListRequest))
        .map(|r| r.count)
        .unwrap_or(1);
    assert_eq!(n, 0);
}

/// @covers: Repository::count — returns correct count after saves
#[test]
fn test_count_after_saves_returns_correct_count_happy() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "a".into(),
    }))
    .unwrap_or_default();
    block_on(repo.save(RepositorySaveRequest {
        id: 2,
        entity: "b".into(),
    }))
    .unwrap_or_default();
    let n = block_on(repo.count(RepositoryListRequest))
        .map(|r| r.count)
        .unwrap_or(0);
    assert_eq!(n, 2);
}

/// @covers: Repository::list_page — returns correct page slice
#[test]
fn test_list_page_returns_correct_slice_happy() {
    let repo = make();
    for i in 0u32..5 {
        block_on(repo.save(RepositorySaveRequest {
            id: i,
            entity: format!("item-{i}"),
        }))
        .unwrap_or_default();
    }
    let page = block_on(repo.list_page(RepositoryListPageRequest {
        offset: 0,
        limit: 2,
    }))
    .map(|r| r.page)
    .unwrap_or_else(|_| edge_application_repository::Page::new(vec![], 0, 0, 2));
    assert_eq!(page.total, 5);
    assert_eq!(page.items.len(), 2);
}

/// @covers: Repository::list_page — offset beyond total returns empty
#[test]
fn test_list_page_offset_beyond_total_returns_empty_error() {
    let repo = make();
    block_on(repo.save(RepositorySaveRequest {
        id: 1,
        entity: "a".into(),
    }))
    .unwrap_or_default();
    let page = block_on(repo.list_page(RepositoryListPageRequest {
        offset: 10,
        limit: 5,
    }))
    .map(|r| r.page)
    .unwrap_or_else(|_| edge_application_repository::Page::new(vec!["x".into()], 0, 10, 5));
    assert!(page.items.is_empty());
}

/// @covers: Repository::list_page — last partial page has_more is false
#[test]
fn test_list_page_last_partial_page_has_no_more_edge() {
    let repo = make();
    for i in 0u32..3 {
        block_on(repo.save(RepositorySaveRequest {
            id: i,
            entity: format!("x{i}"),
        }))
        .unwrap_or_default();
    }
    let page = block_on(repo.list_page(RepositoryListPageRequest {
        offset: 0,
        limit: 10,
    }))
    .map(|r| r.page)
    .unwrap_or_else(|_| edge_application_repository::Page::new(vec![], 0, 0, 10));
    assert!(!page.has_more());
}
