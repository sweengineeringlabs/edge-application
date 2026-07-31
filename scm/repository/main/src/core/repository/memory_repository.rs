//! `Repository` and `QueryableRepository` impls for [`MemoryRepository`].

use futures::future::BoxFuture;

use std::collections::HashMap;
use std::hash::Hash;

use parking_lot::RwLock;

use crate::api::MemoryRepository;
use crate::api::RepositoryError;
use crate::api::{QueryableRepository, Repository};
use crate::api::{
    RepositoryDeleteResponse, RepositoryFindResponse, RepositoryIdRequest, RepositoryListRequest,
    RepositoryListResponse, RepositorySaveRequest,
};

impl<T, Id> MemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    /// Creates a new, empty `MemoryRepository`.
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl<T, Id> Default for MemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Id> Repository for MemoryRepository<T, Id>
where
    T: Clone + Send + Sync + 'static,
    Id: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
    type Entity = T;
    type Id = Id;

    fn find<'a>(
        &'a self,
        req: RepositoryIdRequest<'a, Id>,
    ) -> BoxFuture<'a, Result<RepositoryFindResponse<T>, RepositoryError>> {
        let found = self.store.read().get(req.id).cloned();
        Box::pin(async move { Ok(RepositoryFindResponse { entity: found }) })
    }

    fn save(
        &self,
        req: RepositorySaveRequest<Id, T>,
    ) -> BoxFuture<'_, Result<(), RepositoryError>> {
        self.store.write().insert(req.id, req.entity);
        Box::pin(async move { Ok(()) })
    }

    fn delete<'a>(
        &'a self,
        req: RepositoryIdRequest<'a, Id>,
    ) -> BoxFuture<'a, Result<RepositoryDeleteResponse, RepositoryError>> {
        let removed = self.store.write().remove(req.id).is_some();
        Box::pin(async move { Ok(RepositoryDeleteResponse { removed }) })
    }

    fn list(
        &self,
        _req: RepositoryListRequest,
    ) -> BoxFuture<'_, Result<RepositoryListResponse<T>, RepositoryError>> {
        let items: Vec<T> = self.store.read().values().cloned().collect();
        Box::pin(async move { Ok(RepositoryListResponse { items }) })
    }
}

impl<T, Id> QueryableRepository for MemoryRepository<T, Id>
where
    T: Clone + Send + Sync + 'static,
    Id: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Spec, SpecMatchesRequest, SpecMatchesResponse, SpecRequest};
    use futures::executor::block_on;

    #[test]
    fn test_new_creates_empty_store_happy() {
        let repo = MemoryRepository::<String, String>::new();
        assert!(repo.store.read().is_empty());
    }

    #[test]
    fn test_default_creates_empty_store_edge() {
        let repo = MemoryRepository::<u32, u32>::default();
        assert!(repo.store.read().is_empty());
    }

    #[test]
    fn test_new_independent_instances_are_isolated_error() {
        let a = MemoryRepository::<u32, u32>::new();
        let b = MemoryRepository::<u32, u32>::new();
        a.store.write().insert(1, 100);
        assert!(!b.store.read().contains_key(&1));
    }

    fn repo() -> MemoryRepository<String, u32> {
        MemoryRepository::new()
    }

    #[test]
    fn test_save_then_find_returns_entity_happy() {
        let r = repo();
        block_on(r.save(RepositorySaveRequest {
            id: 1,
            entity: "hello".into(),
        }))
        .unwrap_or_default();
        let found = block_on(r.find(RepositoryIdRequest { id: &1 }))
            .map(|resp| resp.entity)
            .unwrap_or(None);
        assert_eq!(found.as_deref(), Some("hello"));
    }

    #[test]
    fn test_find_missing_id_returns_none_error() {
        let r = repo();
        let found = block_on(r.find(RepositoryIdRequest { id: &99 }))
            .map(|resp| resp.entity)
            .unwrap_or(Some("x".into()));
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_existing_entity_returns_true_happy() {
        let r = repo();
        block_on(r.save(RepositorySaveRequest {
            id: 2,
            entity: "bye".into(),
        }))
        .unwrap_or_default();
        let removed = block_on(r.delete(RepositoryIdRequest { id: &2 }))
            .map(|resp| resp.removed)
            .unwrap_or(false);
        assert!(removed);
    }

    #[test]
    fn test_delete_missing_entity_returns_false_error() {
        let r = repo();
        let removed = block_on(r.delete(RepositoryIdRequest { id: &42 }))
            .map(|resp| resp.removed)
            .unwrap_or(true);
        assert!(!removed);
    }

    #[test]
    fn test_list_empty_repo_returns_empty_vec_edge() {
        let r = repo();
        let items = block_on(r.list(RepositoryListRequest))
            .map(|resp| resp.items)
            .unwrap_or_else(|_| vec!["x".into()]);
        assert!(items.is_empty());
    }

    #[test]
    fn test_find_by_spec_filters_correctly_happy() {
        struct MemoryRepositoryStartsWithASpec;
        impl Spec for MemoryRepositoryStartsWithASpec {
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
        let r = MemoryRepository::new();
        block_on(r.save(RepositorySaveRequest {
            id: 1u32,
            entity: "alpha".into(),
        }))
        .unwrap_or_default();
        block_on(r.save(RepositorySaveRequest {
            id: 2u32,
            entity: "beta".into(),
        }))
        .unwrap_or_default();
        let results = block_on(r.find_by(SpecRequest {
            spec: Box::new(MemoryRepositoryStartsWithASpec),
        }))
        .map(|resp| resp.items)
        .unwrap_or_default();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "alpha");
    }

    #[test]
    fn test_find_one_by_no_match_returns_none_error() {
        struct MemoryRepositoryNeverMatchSpec;
        impl Spec for MemoryRepositoryNeverMatchSpec {
            type Entity = String;
        }
        let r: MemoryRepository<String, u32> = MemoryRepository::new();
        let found = block_on(r.find_one_by(SpecRequest {
            spec: Box::new(MemoryRepositoryNeverMatchSpec),
        }))
        .map(|resp| resp.entity)
        .unwrap_or(Some("x".into()));
        assert!(found.is_none());
    }

    #[test]
    fn test_count_by_matches_correct_count_edge() {
        struct MemoryRepositoryStartsWithASpec;
        impl Spec for MemoryRepositoryStartsWithASpec {
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
        let r = MemoryRepository::new();
        block_on(r.save(RepositorySaveRequest {
            id: 1u32,
            entity: "ant".into(),
        }))
        .unwrap_or_default();
        block_on(r.save(RepositorySaveRequest {
            id: 2u32,
            entity: "bear".into(),
        }))
        .unwrap_or_default();
        block_on(r.save(RepositorySaveRequest {
            id: 3u32,
            entity: "ape".into(),
        }))
        .unwrap_or_default();
        let n = block_on(r.count_by(SpecRequest {
            spec: Box::new(MemoryRepositoryStartsWithASpec),
        }))
        .map(|resp| resp.count)
        .unwrap_or(0);
        assert_eq!(n, 2);
    }
}
