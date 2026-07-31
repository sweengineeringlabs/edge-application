//! [`DeregisterHandlerResponse`] — response for [`HandlerRegistry::deregister`](crate::api::handler::traits::HandlerRegistry::deregister).

/// Whether the deregistered handler existed.
pub struct DeregisterHandlerResponse {
    /// `true` if a handler with the given id was present and removed.
    pub was_present: bool,
}
