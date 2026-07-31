//! `EntityError` — reserved error namespace for the entity contract.

/// Errors that entity operations may produce.
///
/// Entity trait implementations are currently infallible. This enum is
/// `#[non_exhaustive]` so that future variants can be added without a breaking
/// change to consumers that match on it.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum EntityError {}
