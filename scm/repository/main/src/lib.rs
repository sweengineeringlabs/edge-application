//! # edge-domain-repository
//!
//! The `Repository` port contract — data access with MemoryRepository, Page, and Spec.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::AlwaysMatchSpec;
pub use api::CountByResponse;
pub use api::MemoryRepository;
pub use api::MatchingEntitiesResponse;
pub use api::MatchingEntityResponse;
pub use api::Page;
pub use api::RepositoryCountResponse;
pub use api::RepositoryDeleteResponse;
pub use api::RepositoryError;
pub use api::RepositoryExistsResponse;
pub use api::RepositoryFindResponse;
pub use api::RepositoryIdRequest;
pub use api::RepositoryListPageRequest;
pub use api::RepositoryListPageResponse;
pub use api::RepositoryListRequest;
pub use api::RepositoryListResponse;
pub use api::RepositorySaveRequest;
pub use api::SpecMatchesRequest;
pub use api::SpecMatchesResponse;
pub use api::SpecRequest;
pub use saf::QueryableRepository;
pub use saf::Repository;
pub use saf::Spec;
pub use saf::QUERYABLE_REPOSITORY_SVC;
pub use saf::QUERYABLE_REPOSITORY_SVC_FACTORY;
pub use saf::REPOSITORY_SVC;
pub use saf::REPOSITORY_SVC_FACTORY;
pub use saf::SPEC_SVC;
pub use saf::SPEC_SVC_FACTORY;
