//! Integration tests for default domain implementations:
//! direct_command_bus, noop_event_publisher.
#![cfg(all(feature = "event", feature = "command"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::DomainRuntime;
use edge_application::{
    Command, CommandBus, CommandError, Domain, DomainEvent, EventAggregateIdRequest,
    EventAggregateIdResponse, EventError, EventOccurredAtRequest, EventOccurredAtResponse,
    EventPublisher, EventPublisherPublishRequest, EventTypeRequest, EventTypeResponse,
};
use edge_application::{DirectCommandBusRequest, NoopEventPublisherRequest};
use edge_application_command::{
    CommandDispatchRequest, ExecutionRequest, NameRequest, NameResponse,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::SystemTime;

// ── fixtures ─────────────────────────────────────────────────────────────────

struct OkCommand;
impl Command for OkCommand {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "ok".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct ErrCommand;
impl Command for ErrCommand {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "err".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
    }
}

struct AnyEvent;
impl DomainEvent for AnyEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.event",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "agg-1",
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::now(),
        })
    }
}

struct FailingPublisher;
impl EventPublisher for FailingPublisher {
    fn publish(
        &self,
        _req: EventPublisherPublishRequest<'_>,
    ) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Err(EventError::Unavailable("bus down".into())) })
    }
}

// ── direct_command_bus ────────────────────────────────────────────────────────

/// @covers: direct_command_bus
#[tokio::test]
async fn test_direct_command_bus_dispatches_ok_command_successfully() {
    let bus: Arc<dyn CommandBus> = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(OkCommand)
        })
        .await
        .is_ok());
}

/// @covers: direct_command_bus
#[tokio::test]
async fn test_direct_command_bus_propagates_command_error() {
    let bus: Arc<dyn CommandBus> = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(ErrCommand)
        })
        .await
        .is_err());
}

// ── noop_event_publisher ──────────────────────────────────────────────────────

/// @covers: noop_event_publisher
#[tokio::test]
async fn test_noop_event_publisher_always_returns_ok() {
    let pub_: Arc<dyn EventPublisher> = Domain
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap()
        .publisher;
    assert!(pub_
        .publish(EventPublisherPublishRequest { event: &AnyEvent })
        .await
        .is_ok());
}

/// @covers: EventPublisher — custom impl can return error
#[tokio::test]
async fn test_event_publisher_trait_custom_impl_can_return_error() {
    let pub_: Arc<dyn EventPublisher> = Arc::new(FailingPublisher);
    assert!(pub_
        .publish(EventPublisherPublishRequest { event: &AnyEvent })
        .await
        .is_err());
}
