//! API layer — domain aggregation re-exports.

#[cfg(test)]
mod tests;

// ── domain (never extracted; always internal) ─────────────────────────────────
mod domain;
#[cfg(feature = "command")]
pub use domain::{DirectCommandBusRequest, DirectCommandBusResponse};
pub use domain::{
    Domain, DomainError, DomainExtension, DomainExtensionHealthRequest, DomainRuntime,
    MemoryOutboundRegistry, NoopDomainExtension, OutboundDeregisterRequest,
    OutboundDeregisterResponse, OutboundGetRequest, OutboundGetResponse, OutboundIsEmptyRequest,
    OutboundIsEmptyResponse, OutboundLenRequest, OutboundLenResponse, OutboundNamesRequest,
    OutboundNamesResponse, OutboundRegisterRequest, OutboundRegisterResponse, OutboundRegistry,
};
#[cfg(feature = "event")]
pub use domain::{
    InProcessEventBusRequest, InProcessEventBusResponse, NoopEventBusRequest, NoopEventBusResponse,
    NoopEventPublisherRequest, NoopEventPublisherResponse,
};

// ── spi ───────────────────────────────────────────────────────────────────────
mod spi;
pub use spi::{DomainAssemblyHook, NoopDomainAssemblyHook};

// ── base ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "base")]
pub use edge_application_base::Request;
#[cfg(feature = "base")]
pub use edge_application_base::Response;

// ── command ───────────────────────────────────────────────────────────────────
#[cfg(feature = "command")]
pub use edge_application_command::CommandBus;
#[cfg(feature = "command")]
pub use edge_application_command::DirectCommandBus;

// ── event ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_application_event::Aggregate;
#[cfg(feature = "event")]
pub use edge_application_event::AggregateApplyRequest;
#[cfg(feature = "event")]
pub use edge_application_event::AggregateApplyResponse;
#[cfg(feature = "event")]
pub use edge_application_event::AggregateIdentityRequest;
#[cfg(feature = "event")]
pub use edge_application_event::AggregateIdentityResponse;
#[cfg(feature = "event")]
pub use edge_application_event::DomainEvent;
#[cfg(feature = "event")]
pub use edge_application_event::EventAggregateIdRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventAggregateIdResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventBus;
#[cfg(feature = "event")]
pub use edge_application_event::EventBusConfig;
#[cfg(feature = "event")]
pub use edge_application_event::EventBusPublishRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventBusSubscribeRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventBusSubscribeResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventError;
#[cfg(feature = "event")]
pub use edge_application_event::EventOccurredAtRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventOccurredAtResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventPublisher;
#[cfg(feature = "event")]
pub use edge_application_event::EventPublisherPublishRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventSourceRecvNextRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventSourceRecvNextResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventStore;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreAppendRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreAppendResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreError;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreLoadFromRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreLoadFromResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreLoadRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventStoreLoadResponse;
#[cfg(feature = "event")]
pub use edge_application_event::EventTypeRequest;
#[cfg(feature = "event")]
pub use edge_application_event::EventTypeResponse;
#[cfg(feature = "event")]
pub use edge_application_event::MemoryEventStore;
#[cfg(feature = "event")]
pub use edge_application_event::InProcessEventBus;
#[cfg(feature = "event")]
pub use edge_application_event::NoopEventBus;
#[cfg(feature = "event")]
pub use edge_application_event::NoopEventPublisher;

// ── handler ───────────────────────────────────────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_application_handler::EchoHandler;
#[cfg(feature = "handler")]
pub use edge_application_handler::Handler;
#[cfg(feature = "handler")]
pub use edge_application_handler::HandlerRegistry;
#[cfg(feature = "handler")]
pub use edge_application_handler::InProcessHandlerRegistry;

// ── projection ────────────────────────────────────────────────────────────────
#[cfg(feature = "projection")]
pub use edge_application_projection::MemoryProjection;
#[cfg(feature = "projection")]
pub use edge_application_projection::Projection;
#[cfg(feature = "projection")]
pub use edge_application_projection::ProjectionApplyRequest;
#[cfg(feature = "projection")]
pub use edge_application_projection::ProjectionReadModelRequest;
#[cfg(feature = "projection")]
pub use edge_application_projection::ProjectionReadModelResponse;

// ── query ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_application_query::DirectQueryBus;
#[cfg(feature = "query")]
pub use edge_application_query::QueryBus;
#[cfg(feature = "query")]
pub use edge_application_query::QueryDispatchRequest;
#[cfg(feature = "query")]
pub use edge_application_query::QueryExecuteRequest;
#[cfg(feature = "query")]
pub use edge_application_query::QueryNameRequest;
#[cfg(feature = "query")]
pub use edge_application_query::QueryNameResponse;
#[cfg(feature = "query")]
pub use edge_application_query::QueryResultResponse;

// ── repository ────────────────────────────────────────────────────────────────
#[cfg(feature = "repository")]
pub use edge_application_repository::AlwaysMatchSpec;
#[cfg(feature = "repository")]
pub use edge_application_repository::CountByResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::MemoryRepository;
#[cfg(feature = "repository")]
pub use edge_application_repository::MatchingEntitiesResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::MatchingEntityResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::QueryableRepository;
#[cfg(feature = "repository")]
pub use edge_application_repository::Repository;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryCountResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryDeleteResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryExistsResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryFindResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryIdRequest;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryListPageRequest;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryListPageResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryListRequest;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryListResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositorySaveRequest;
#[cfg(feature = "repository")]
pub use edge_application_repository::SpecMatchesRequest;
#[cfg(feature = "repository")]
pub use edge_application_repository::SpecMatchesResponse;
#[cfg(feature = "repository")]
pub use edge_application_repository::SpecRequest;

// ── saga ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "saga")]
pub use edge_application_saga::MemorySagaStore;
#[cfg(feature = "saga")]
pub use edge_application_saga::Saga;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaStore;

// ── snapshot ──────────────────────────────────────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::MemorySnapshotStore;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::Snapshot;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotStore;

// ── validator ─────────────────────────────────────────────────────────────────
#[cfg(feature = "validator")]
pub use edge_application_validator::Validator;
