//! [`RegisterHandlerRequest`] — request for [`HandlerRegistry::register`](crate::api::handler::traits::HandlerRegistry::register).

use std::sync::Arc;

use crate::api::handler::traits::handler::Handler;

/// Request to register a handler, replacing any existing entry with the same id.
///
/// Construction lives in `core::handler` — api/ is a declaration layer only.
pub struct RegisterHandlerRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    pub(crate) handler: Arc<dyn Handler<Request = Req, Response = Resp>>,
}
