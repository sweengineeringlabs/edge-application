//! # edge-domain-event
//!
//! The event port contracts — event sourcing, CQRS event bus, publish/subscribe.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::AggregateApplyRequest;
pub use api::AggregateApplyResponse;
pub use api::AggregateIdentityRequest;
pub use api::AggregateIdentityResponse;
pub use api::ClosedEventSource;
pub use api::EventAggregateIdRequest;
pub use api::EventAggregateIdResponse;
pub use api::EventBusConfig;
pub use api::EventBusPublishRequest;
pub use api::EventBusSubscribeRequest;
pub use api::EventBusSubscribeResponse;
pub use api::EventEnvelope;
pub use api::EventError;
pub use api::EventOccurredAtRequest;
pub use api::EventOccurredAtResponse;
pub use api::EventPublisherPublishRequest;
pub use api::EventSourceRecvNextRequest;
pub use api::EventSourceRecvNextResponse;
pub use api::EventStoreAppendRequest;
pub use api::EventStoreAppendResponse;
pub use api::EventStoreError;
pub use api::EventStoreLoadFromRequest;
pub use api::EventStoreLoadFromResponse;
pub use api::EventStoreLoadRequest;
pub use api::EventStoreLoadResponse;
pub use api::EventTypeRequest;
pub use api::EventTypeResponse;
pub use api::ExpectedVersion;
pub use api::MemoryEventStore;
pub use api::InProcessEventBus;
pub use api::NoopAggregate;
pub use api::NoopDomainEvent;
pub use api::NoopEventBus;
pub use api::NoopEventPublisher;
pub use saf::Aggregate;
pub use saf::DomainEvent;
pub use saf::EventBus;
pub use saf::EventPublisher;
pub use saf::EventSource;
pub use saf::EventStore;
pub use saf::AGGREGATE_SVC;
pub use saf::AGGREGATE_SVC_FACTORY;
pub use saf::DOMAIN_EVENT_SVC;
pub use saf::DOMAIN_EVENT_SVC_FACTORY;
pub use saf::EVENT_BUS_SVC;
pub use saf::EVENT_BUS_SVC_FACTORY;
pub use saf::EVENT_PUBLISHER_SVC;
pub use saf::EVENT_PUBLISHER_SVC_FACTORY;
pub use saf::EVENT_SOURCE_SVC;
pub use saf::EVENT_SOURCE_SVC_FACTORY;
pub use saf::EVENT_STORE_SVC;
pub use saf::EVENT_STORE_SVC_FACTORY;
