//! [`QueryNameResponse`] — wrapper for a query's stable name.

/// Result of [`Query::name`](crate::api::query::traits::Query::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueryNameResponse<'a> {
    /// Stable identifier for this query type.
    pub name: &'a str,
}
