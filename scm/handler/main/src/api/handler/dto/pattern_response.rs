//! [`PatternResponse`] — response for [`Handler::pattern`](crate::api::handler::traits::Handler::pattern).

/// The route pattern a handler matches.
pub struct PatternResponse {
    /// The route pattern this handler matches.
    pub pattern: String,
}
