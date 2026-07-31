//! [`EmptyRequest`] — canonical zero-sized "no payload" request.

/// A request that carries no data.
///
/// `Request`/`Response` are independent bounds — a `Handler`/`Service` may pair `EmptyRequest`
/// with any real response type, or vice versa with [`EmptyResponse`](super::EmptyResponse); the
/// two are never required to be used together. Provided so downstream crates reaching for "no
/// payload" don't each need to declare their own local marker type for the same concept (e.g.
/// `domain-service`'s `NoopRequest` predates this and is domain-service-specific; this is the
/// shared, foundational equivalent).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EmptyRequest;
