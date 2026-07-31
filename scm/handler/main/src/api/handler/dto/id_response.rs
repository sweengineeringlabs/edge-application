//! [`IdResponse`] — response for [`Handler::id`](crate::api::handler::traits::Handler::id).

/// A handler's stable identifier.
pub struct IdResponse {
    /// The handler's stable identifier.
    pub id: String,
}
