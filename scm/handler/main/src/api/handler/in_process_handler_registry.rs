//! [`InProcessHandlerRegistry`] — in-process, thread-safe handler registry.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::handler::traits::Handler;

/// An in-process, thread-safe handler registry backed by a `RwLock<HashMap>`.
///
/// The `HandlerRegistry` trait implementation, `Default`, and inherent methods live in
/// `core::handler::in_process_handler_registry`.
pub struct InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The backing store.
    pub(crate) handlers: RwLock<HashMap<String, Arc<dyn Handler<Request = Req, Response = Resp>>>>,
}
