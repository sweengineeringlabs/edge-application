//! `QueryableRepository` — specification-based query extension for `Repository`.

use std::future::Future;
use std::pin::Pin;

use crate::api::repository::errors::RepositoryError;
use crate::api::repository::traits::Repository;
use crate::api::repository::dto::{
    CountByResponse, MatchingEntitiesResponse, MatchingEntityResponse, RepositoryListRequest,
    SpecMatchesRequest, SpecRequest,
};

/// Extends [`Repository`] with specification-based query methods.
///
/// The `Self::Entity: Clone` supertrait bound is required because the default
/// implementations load the full entity list and clone matching entries.
/// Concrete implementations may override these with more efficient queries.
pub trait QueryableRepository: Repository
where
    Self::Entity: Clone + Send + Sync + 'static,
{
    /// Returns all entities that satisfy the given specification.
    fn find_by(
        &self,
        req: SpecRequest<Self::Entity>,
    ) -> Pin<Box<dyn Future<Output = Result<MatchingEntitiesResponse<Self::Entity>, RepositoryError>> + Send + '_>> {
        Box::pin(async move {
            let all = self.list(RepositoryListRequest).await?.items;
            let mut items = Vec::new();
            for e in all {
                if req.spec.matches(SpecMatchesRequest { entity: &e })?.matches {
                    items.push(e);
                }
            }
            Ok(MatchingEntitiesResponse { items })
        })
    }

    /// Returns the first entity that satisfies the given specification, or `None`.
    fn find_one_by(
        &self,
        req: SpecRequest<Self::Entity>,
    ) -> Pin<Box<dyn Future<Output = Result<MatchingEntityResponse<Self::Entity>, RepositoryError>> + Send + '_>> {
        Box::pin(async move {
            let all = self.list(RepositoryListRequest).await?.items;
            let mut found = None;
            for e in all {
                if req.spec.matches(SpecMatchesRequest { entity: &e })?.matches {
                    found = Some(e);
                    break;
                }
            }
            Ok(MatchingEntityResponse { entity: found })
        })
    }

    /// Returns the count of entities that satisfy the given specification.
    fn count_by(
        &self,
        req: SpecRequest<Self::Entity>,
    ) -> Pin<Box<dyn Future<Output = Result<CountByResponse, RepositoryError>> + Send + '_>> {
        Box::pin(async move {
            let all = self.list(RepositoryListRequest).await?.items;
            let mut count = 0;
            for e in &all {
                if req.spec.matches(SpecMatchesRequest { entity: e })?.matches {
                    count += 1;
                }
            }
            Ok(CountByResponse { count })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::repository::traits::Spec;
    use crate::api::repository::dto::{
        RepositoryDeleteResponse, RepositoryFindResponse, RepositoryIdRequest,
        RepositoryListResponse, RepositorySaveRequest, SpecMatchesResponse,
    };
    use futures::executor::block_on;

    struct VecRepo {
        items: Vec<u32>,
    }

    impl Repository for VecRepo {
        type Entity = u32;
        type Id = usize;

        fn find<'a>(
            &'a self,
            req: RepositoryIdRequest<'a, usize>,
        ) -> Pin<Box<dyn Future<Output = Result<RepositoryFindResponse<u32>, RepositoryError>> + Send + 'a>> {
            let val = self.items.get(*req.id).copied();
            Box::pin(async move { Ok(RepositoryFindResponse { entity: val }) })
        }
        fn save(
            &self,
            _req: RepositorySaveRequest<usize, u32>,
        ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + '_>> {
            Box::pin(async move { Ok(()) })
        }
        fn delete<'a>(
            &'a self,
            _req: RepositoryIdRequest<'a, usize>,
        ) -> Pin<Box<dyn Future<Output = Result<RepositoryDeleteResponse, RepositoryError>> + Send + 'a>> {
            Box::pin(async move { Ok(RepositoryDeleteResponse { removed: false }) })
        }
        fn list(
            &self,
            _req: RepositoryListRequest,
        ) -> Pin<Box<dyn Future<Output = Result<RepositoryListResponse<u32>, RepositoryError>> + Send + '_>> {
            let vals = self.items.clone();
            Box::pin(async move { Ok(RepositoryListResponse { items: vals }) })
        }
    }

    impl QueryableRepository for VecRepo {}

    struct EvenSpec;
    impl Spec for EvenSpec {
        type Entity = u32;

        fn matches(
            &self,
            req: SpecMatchesRequest<'_, u32>,
        ) -> Result<SpecMatchesResponse, RepositoryError> {
            Ok(SpecMatchesResponse {
                matches: req.entity.is_multiple_of(2),
            })
        }
    }

    #[test]
    fn test_find_by_matching_spec_returns_filtered_results_happy() {
        let repo = VecRepo {
            items: vec![1, 2, 3, 4],
        };
        let results = block_on(repo.find_by(SpecRequest {
            spec: Box::new(EvenSpec),
        }))
        .map(|r| r.items)
        .unwrap_or_default();
        assert_eq!(results, vec![2, 4]);
    }

    #[test]
    fn test_find_one_by_matching_spec_returns_first_happy() {
        let repo = VecRepo {
            items: vec![1, 2, 3, 4],
        };
        let result = block_on(repo.find_one_by(SpecRequest {
            spec: Box::new(EvenSpec),
        }))
        .map(|r| r.entity)
        .unwrap_or(None);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_count_by_no_match_returns_zero_edge() {
        let repo = VecRepo {
            items: vec![1, 3, 5],
        };
        let n = block_on(repo.count_by(SpecRequest {
            spec: Box::new(EvenSpec),
        }))
        .map(|r| r.count)
        .unwrap_or(1);
        assert_eq!(n, 0);
    }
}
