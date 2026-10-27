//! Object-safe runtime seam for `Domain`'s non-generic constructors.

#[cfg(feature = "command")]
use crate::api::domain::dto::{DirectCommandBusRequest, DirectCommandBusResponse};
#[cfg(feature = "event")]
use crate::api::domain::dto::{
    InProcessEventBusRequest, InProcessEventBusResponse, NoopEventBusRequest, NoopEventBusResponse,
    NoopEventPublisherRequest, NoopEventPublisherResponse,
};
#[cfg(any(feature = "command", feature = "event"))]
use crate::api::domain::errors::DomainError;

/// Instance-based factory seam for [`Domain`](crate::api::domain::domain::Domain)'s
/// constructors that take no caller-supplied type parameters.
///
/// Generic constructors (`echo_handler::<T>`, `new_in_memory_repository::<T,
/// Id>`, ...) stay inherent methods on `Domain` — a generic method cannot be
/// part of an object-safe trait. This trait carries the subset that *can* be
/// object-safe, so `Domain` is a real `dyn DomainRuntime` injection seam
/// rather than merely a static namespace.
pub trait DomainRuntime: Send + Sync {
    /// Construct a [`CommandBus`](crate::api::CommandBus) that dispatches commands inline.
    #[cfg(feature = "command")]
    fn direct_command_bus(
        &self,
        req: DirectCommandBusRequest,
    ) -> Result<DirectCommandBusResponse, DomainError>;

    /// Construct an [`EventPublisher`](crate::api::EventPublisher) that discards all events silently.
    #[cfg(feature = "event")]
    fn noop_event_publisher(
        &self,
        req: NoopEventPublisherRequest,
    ) -> Result<NoopEventPublisherResponse, DomainError>;

    /// Construct an in-process broadcast-backed [`EventBus`](crate::api::EventBus).
    #[cfg(feature = "event")]
    fn in_process_event_bus(
        &self,
        req: InProcessEventBusRequest,
    ) -> Result<InProcessEventBusResponse, DomainError>;

    /// Construct an [`EventBus`](crate::api::EventBus) that silently discards all events.
    #[cfg(feature = "event")]
    fn noop_event_bus(&self, req: NoopEventBusRequest)
        -> Result<NoopEventBusResponse, DomainError>;
}
