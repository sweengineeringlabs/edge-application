//! [`HandlerLookupRequest`] — request for [`HandlerRegistry::get`](crate::api::handler::traits::HandlerRegistry::get).

/// Request to look up a handler by id.
pub struct HandlerLookupRequest {
    /// The id of the handler to look up.
    pub id: String,
}
