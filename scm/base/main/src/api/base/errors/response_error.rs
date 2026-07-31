//! `ResponseError` — reserved error namespace for `Response::validate`.

/// Errors that [`Response::validate`](crate::api::Response::validate) may produce.
///
/// The default implementation is currently infallible. This enum is
/// `#[non_exhaustive]` so that future variants can be added without a breaking
/// change to consumers that match on it.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum ResponseError {}
