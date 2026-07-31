//! [`ValidationResponse`] — wrapper for successful `Request`/`Response` validation.

/// Result of [`Request::validate`](crate::api::Request::validate)/
/// [`Response::validate`](crate::api::Response::validate).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ValidationResponse;
