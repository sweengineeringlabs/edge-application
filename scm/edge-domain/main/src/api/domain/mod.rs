//! Domain theme — cross-cutting domain types and the extension hook contract.

#[allow(clippy::module_inception)]
mod domain;
pub mod dto;
pub mod errors;
mod memory_outbound_registry;
mod noop_domain_extension;
pub mod traits;

#[cfg(feature = "command")]
pub use dto::{DirectCommandBusRequest, DirectCommandBusResponse};
#[cfg(feature = "event")]
pub use dto::{
    InProcessEventBusRequest, InProcessEventBusResponse, NoopEventBusRequest, NoopEventBusResponse,
    NoopEventPublisherRequest, NoopEventPublisherResponse,
};
pub use dto::{
    OutboundDeregisterRequest, OutboundDeregisterResponse, OutboundGetRequest, OutboundGetResponse,
    OutboundIsEmptyRequest, OutboundIsEmptyResponse, OutboundLenRequest, OutboundLenResponse,
    OutboundNamesRequest, OutboundNamesResponse, OutboundRegisterRequest, OutboundRegisterResponse,
};
pub use errors::DomainError;
pub use traits::{
    Domain, DomainExtension, DomainExtensionHealthRequest, DomainRuntime, MemoryOutboundRegistry,
    NoopDomainExtension, OutboundRegistry,
};
