//! `Repository` — core async CRUD port contract.

use std::future::Future;
use std::pin::Pin;

use crate::api::repository::dto::{
    RepositoryCountResponse, RepositoryDeleteResponse, RepositoryExistsResponse,
    RepositoryFindResponse, RepositoryIdRequest, RepositoryListPageRequest,
    RepositoryListPageResponse, RepositoryListRequest, RepositoryListResponse,
    RepositorySaveRequest,
};
use crate::api::repository::errors::RepositoryError;
use crate::api::repository::page::Page;

/// Boxed, sendable future used by the repository contract.
pub type RepositoryFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, RepositoryError>> + Send + 'a>>;

/// Core async CRUD contract for a typed repository.
///
/// Associated types enforce that each implementor is bound to exactly one
/// entity type and one id type — a `UserRepository` can never accidentally
/// store a `Product`.
///
/// All methods return `BoxFuture` so implementations are not required to use
/// `#[async_trait]`.  Default methods for `exists`, `count`, and `list_page`
/// are provided in terms of the required methods.
pub trait Repository: Send + Sync {
    /// The entity type stored in this repository.
    type Entity: Send + 'static;
    /// The identifier type used to key entities.
    type Id: Send + Sync + 'static;

    /// Returns the entity with the given `id`, or `None` if it does not exist.
    fn find<'a>(
        &'a self,
        req: RepositoryIdRequest<'a, Self::Id>,
    ) -> RepositoryFuture<'a, RepositoryFindResponse<Self::Entity>>;

    /// Persists `entity` under `id`, replacing any existing entry.
    fn save(&self, req: RepositorySaveRequest<Self::Id, Self::Entity>) -> RepositoryFuture<'_, ()>;

    /// Removes the entity with the given `id`.
    ///
    /// Returns `true` if an entry was removed, `false` if it did not exist.
    fn delete<'a>(
        &'a self,
        req: RepositoryIdRequest<'a, Self::Id>,
    ) -> RepositoryFuture<'a, RepositoryDeleteResponse>;

    /// Returns all entities in the repository.
    fn list(
        &self,
        req: RepositoryListRequest,
    ) -> RepositoryFuture<'_, RepositoryListResponse<Self::Entity>>;

    /// Returns `true` if an entity with the given `id` exists.
    fn exists<'a>(
        &'a self,
        req: RepositoryIdRequest<'a, Self::Id>,
    ) -> RepositoryFuture<'a, RepositoryExistsResponse> {
        Box::pin(async move {
            let found = self.find(req).await?.entity.is_some();
            Ok(RepositoryExistsResponse { exists: found })
        })
    }

    /// Returns the total number of entities in the repository.
    fn count(&self, req: RepositoryListRequest) -> RepositoryFuture<'_, RepositoryCountResponse> {
        Box::pin(async move {
            let count = self.list(req).await?.items.len();
            Ok(RepositoryCountResponse { count })
        })
    }

    /// Returns a paginated slice of entities.
    fn list_page(
        &self,
        req: RepositoryListPageRequest,
    ) -> RepositoryFuture<'_, RepositoryListPageResponse<Self::Entity>>
    where
        Self::Entity: Clone,
        Self: Sized,
    {
        Box::pin(async move {
            let all = self.list(RepositoryListRequest).await?.items;
            let total = all.len();
            let items = all.into_iter().skip(req.offset).take(req.limit).collect();
            Ok(RepositoryListPageResponse {
                page: Page::new(items, total, req.offset, req.limit),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    struct MapRepo {
        store: std::sync::Mutex<std::collections::HashMap<u32, String>>,
    }

    impl MapRepo {
        fn new() -> Self {
            Self {
                store: std::sync::Mutex::new(std::collections::HashMap::new()),
            }
        }
    }

    impl Repository for MapRepo {
        type Entity = String;
        type Id = u32;

        fn find<'a>(
            &'a self,
            req: RepositoryIdRequest<'a, u32>,
        ) -> RepositoryFuture<'a, RepositoryFindResponse<String>> {
            let val = self
                .store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(req.id)
                .cloned();
            Box::pin(async move { Ok(RepositoryFindResponse { entity: val }) })
        }

        fn save(&self, req: RepositorySaveRequest<u32, String>) -> RepositoryFuture<'_, ()> {
            self.store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(req.id, req.entity);
            Box::pin(async move { Ok(()) })
        }

        fn delete<'a>(
            &'a self,
            req: RepositoryIdRequest<'a, u32>,
        ) -> RepositoryFuture<'a, RepositoryDeleteResponse> {
            let removed = self
                .store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(req.id)
                .is_some();
            Box::pin(async move { Ok(RepositoryDeleteResponse { removed }) })
        }

        fn list(
            &self,
            _req: RepositoryListRequest,
        ) -> RepositoryFuture<'_, RepositoryListResponse<String>> {
            let vals: Vec<_> = self
                .store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .values()
                .cloned()
                .collect();
            Box::pin(async move { Ok(RepositoryListResponse { items: vals }) })
        }
    }

    #[test]
    fn test_exists_saved_entity_returns_true_happy() {
        let repo = MapRepo::new();
        block_on(repo.save(RepositorySaveRequest {
            id: 1,
            entity: "alpha".into(),
        }))
        .unwrap_or_default();
        assert!(block_on(repo.exists(RepositoryIdRequest { id: &1 }))
            .map(|r| r.exists)
            .unwrap_or(false));
    }

    #[test]
    fn test_exists_missing_entity_returns_false_error() {
        let repo = MapRepo::new();
        assert!(!block_on(repo.exists(RepositoryIdRequest { id: &99 }))
            .map(|r| r.exists)
            .unwrap_or(true));
    }

    #[test]
    fn test_count_empty_repo_returns_zero_edge() {
        let repo = MapRepo::new();
        let n = block_on(repo.count(RepositoryListRequest))
            .map(|r| r.count)
            .unwrap_or(1);
        assert_eq!(n, 0);
    }
}
