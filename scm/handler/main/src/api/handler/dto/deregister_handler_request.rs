//! [`DeregisterHandlerRequest`] — request for [`HandlerRegistry::deregister`](crate::api::handler::traits::HandlerRegistry::deregister).

/// Request to remove the handler with the given id.
pub struct DeregisterHandlerRequest {
    /// The id of the handler to remove.
    pub id: String,
}
