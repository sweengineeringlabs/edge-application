//! [`EmptinessResponse`] — response for [`HandlerRegistry::is_empty`](crate::api::handler::traits::HandlerRegistry::is_empty).

/// Whether no handlers are registered.
pub struct EmptinessResponse {
    /// `true` if no handlers are registered.
    pub empty: bool,
}
