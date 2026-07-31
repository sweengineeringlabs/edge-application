//! [`LenResponse`] — response for [`HandlerRegistry::len`](crate::api::handler::traits::HandlerRegistry::len).

/// The number of registered handlers.
pub struct LenResponse {
    /// The number of registered handlers.
    pub count: usize,
}
