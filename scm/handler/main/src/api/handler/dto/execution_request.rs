//! [`ExecutionRequest`] — request for [`Handler::execute`](crate::api::handler::traits::Handler::execute).

use crate::api::handler::vo::handler_context::HandlerContext;

/// Request to execute a handler with the given request payload and request-scoped context.
pub struct ExecutionRequest<'a, Req> {
    /// The handler-specific request payload.
    pub req: Req,
    /// The request-scoped execution context.
    pub ctx: &'a HandlerContext<'a>,
}
