//! `RequestError` — reserved error namespace for `Request::validate`.

/// Errors that [`Request::validate`](crate::api::Request::validate) may produce.
///
/// The default implementation is currently infallible. This enum is
/// `#[non_exhaustive]` so that future variants can be added without a breaking
/// change to consumers that match on it.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum RequestError {}
