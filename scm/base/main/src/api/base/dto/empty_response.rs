//! [`EmptyResponse`] — canonical zero-sized "no payload" response.

/// A response that carries no data.
///
/// `Request`/`Response` are independent bounds — a `Handler`/`Service` may pair `EmptyResponse`
/// with any real request type, or vice versa with [`EmptyRequest`](super::EmptyRequest); the two
/// are never required to be used together. Provided so downstream crates reaching for "no
/// payload" don't each need to declare their own local marker type for the same concept (e.g.
/// `domain-service`'s `NoopResponse` predates this and is domain-service-specific; this is the
/// shared, foundational equivalent).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EmptyResponse;
