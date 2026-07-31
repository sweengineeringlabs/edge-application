//! [`HandlerLookupResponse`] — response for [`HandlerRegistry::get`](crate::api::handler::traits::HandlerRegistry::get).

use std::sync::Arc;

use crate::api::handler::traits::handler::Handler;

/// The handler found for the requested id, if any.
pub struct HandlerLookupResponse<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The handler registered under the requested id, if present.
    pub handler: Option<Arc<dyn Handler<Request = Req, Response = Resp>>>,
}
