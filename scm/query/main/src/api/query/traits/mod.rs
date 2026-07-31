//! Query theme — port contracts.

#[allow(clippy::module_inception)]
pub mod query;
pub mod query_bus;

pub use query::Query;
pub use query_bus::QueryBus;
