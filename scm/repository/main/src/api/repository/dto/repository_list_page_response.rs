//! [`RepositoryListPageResponse`] — wrapper for a paginated slice of entities.

use crate::api::repository::page::Page;

/// Result of [`Repository::list_page`](crate::api::repository::traits::Repository::list_page).
#[derive(Debug, Clone)]
pub struct RepositoryListPageResponse<T> {
    /// The requested page.
    ///
    /// SEA `field_type_purity` tradeoff (accepted, tracked in edge-domain#131):
    /// `Page<T>` is a plain-data pagination DTO (items + cursor/total-count
    /// metadata) with no behavior to abstract behind a trait, so there is no
    /// real interface to wrap in `Arc<dyn Trait>`/`Box<dyn Trait>` here.
    /// Manufacturing a trait around it would be ceremony purely to satisfy
    /// the linter, not a genuine layering fix — contrast with a field that
    /// wraps a `dyn Trait`, which is a real, fixable violation of this rule.
    pub page: Page<T>,
}
