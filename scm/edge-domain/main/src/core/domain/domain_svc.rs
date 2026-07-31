//! Domain factory methods — all building-block constructors as instance
//! methods on [`Domain`]. The non-generic subset also implements
//! [`DomainRuntime`], giving `Domain` a real `dyn DomainRuntime` seam.

#[cfg(feature = "repository")]
use std::hash::Hash;
use std::sync::Arc;

use crate::api::Domain;

#[cfg(feature = "command")]
use crate::api::CommandBus;
#[cfg(feature = "command")]
use crate::api::{DirectCommandBusRequest, DirectCommandBusResponse};
#[cfg(feature = "command")]
use edge_application_command::DirectCommandBus;

#[cfg(feature = "event")]
use crate::api::Aggregate;
#[cfg(feature = "event")]
use crate::api::AggregateApplyRequest;
#[cfg(feature = "event")]
use crate::api::DomainEvent;
#[cfg(feature = "event")]
use crate::api::EventBus;
#[cfg(feature = "event")]
use crate::api::EventPublisher;
#[cfg(feature = "event")]
use crate::api::EventStore;
#[cfg(feature = "event")]
use crate::api::EventStoreError;
#[cfg(feature = "event")]
use crate::api::EventStoreLoadRequest;
#[cfg(feature = "event")]
use crate::api::MemoryEventStore;
#[cfg(feature = "event")]
use crate::api::NoopEventBus;
#[cfg(feature = "event")]
use crate::api::NoopEventPublisher;
#[cfg(feature = "event")]
use crate::api::{InProcessEventBusRequest, InProcessEventBusResponse};
#[cfg(feature = "event")]
use crate::api::{NoopEventBusRequest, NoopEventBusResponse};
#[cfg(feature = "event")]
use crate::api::{NoopEventPublisherRequest, NoopEventPublisherResponse};
#[cfg(feature = "event")]
use edge_application_event::InProcessEventBus;

#[cfg(feature = "handler")]
use crate::api::EchoHandler;
#[cfg(feature = "handler")]
use crate::api::Handler;
#[cfg(feature = "handler")]
use crate::api::HandlerRegistry as HandlerRegistryTrait;
#[cfg(feature = "handler")]
use crate::api::InProcessHandlerRegistry;

#[cfg(feature = "projection")]
use crate::api::Projection;
#[cfg(feature = "projection")]
use edge_application_projection::MemoryProjection;

#[cfg(feature = "query")]
use crate::api::DirectQueryBus;
#[cfg(feature = "query")]
use crate::api::QueryBus;

#[cfg(feature = "repository")]
use crate::api::MemoryRepository;
#[cfg(feature = "repository")]
use crate::api::QueryableRepository;
#[cfg(feature = "repository")]
use crate::api::Repository;

#[cfg(feature = "saga")]
use crate::api::MemorySagaStore;
#[cfg(feature = "saga")]
use crate::api::Saga;
#[cfg(feature = "saga")]
use crate::api::SagaStore;

#[cfg(feature = "snapshot")]
use crate::api::Snapshot;
#[cfg(feature = "snapshot")]
use crate::api::SnapshotStore;
#[cfg(feature = "snapshot")]
use edge_application_snapshot::MemorySnapshotStore;

#[cfg(any(feature = "command", feature = "event"))]
use crate::api::DomainError;
use crate::api::DomainRuntime;

impl Domain {
    /// Construct a handler that returns its input unchanged.
    ///
    /// Useful for transport-layer integration tests — verifies routing and
    /// codec wiring without requiring any business logic implementation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use edge_application::Domain;
    ///
    /// #[derive(Clone)]
    /// struct Ping;
    ///
    /// impl edge_application_base::Request for Ping {}
    /// impl edge_application_base::Response for Ping {}
    ///
    /// let h = Domain.echo_handler::<Ping>("echo", "/ping");
    /// ```
    #[cfg(feature = "handler")]
    pub fn echo_handler<T>(
        &self,
        id: impl Into<String>,
        pattern: impl Into<String>,
    ) -> Arc<dyn Handler<Request = T, Response = T>>
    where
        T: Clone + edge_application_base::Request + edge_application_base::Response,
    {
        let id = id.into();
        let pattern = pattern.into();
        Arc::new(EchoHandler::from((id.as_str(), pattern.as_str())))
    }

    /// Construct a fresh empty handler registry.
    ///
    /// Backed by a `parking_lot::RwLock<HashMap>` — safe for concurrent
    /// registration and lookup across threads.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use edge_application::Domain;
    /// use edge_application_handler::EmptinessRequest;
    ///
    /// struct Ping;
    ///
    /// impl edge_application_base::Request for Ping {}
    /// impl edge_application_base::Response for Ping {}
    ///
    /// let registry = Domain.new_handler_registry::<Ping, Ping>();
    /// assert!(registry.is_empty(EmptinessRequest).unwrap().empty);
    /// ```
    #[cfg(feature = "handler")]
    pub fn new_handler_registry<Req, Resp>(
        &self,
    ) -> Arc<dyn HandlerRegistryTrait<Request = Req, Response = Resp>>
    where
        Req: edge_application_base::Request,
        Resp: edge_application_base::Response,
    {
        Arc::new(InProcessHandlerRegistry::default())
    }

    /// Construct a paired `(T, U)` from a shared backend `Arc<B>`.
    ///
    /// Both closures receive `Arc::clone(&backend)`, ensuring writes through
    /// `T` are visible to reads through `U` when both share an in-memory
    /// store. Without this, two separate `from_config()` calls create
    /// independent backend instances and writes are invisible across them.
    pub fn paired<B: ?Sized, T, U>(
        &self,
        backend: Arc<B>,
        make_first: impl FnOnce(Arc<B>) -> T,
        make_second: impl FnOnce(Arc<B>) -> U,
    ) -> (T, U) {
        let first = make_first(Arc::clone(&backend));
        let second = make_second(backend);
        (first, second)
    }

    /// Construct a thread-safe in-memory [`Repository`].
    #[cfg(feature = "repository")]
    pub fn new_in_memory_repository<T, Id>(&self) -> Arc<dyn Repository<Entity = T, Id = Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = MemoryRepository::new();
        Arc::new(r)
    }

    /// Construct a thread-safe in-memory [`QueryableRepository`].
    #[cfg(feature = "repository")]
    pub fn new_in_memory_queryable_repository<T, Id>(
        &self,
    ) -> Arc<dyn QueryableRepository<Entity = T, Id = Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = MemoryRepository::new();
        Arc::new(r)
    }

    /// Construct a thread-safe in-memory [`EventStore`].
    #[cfg(feature = "event")]
    pub fn new_in_memory_event_store<E>(&self) -> Arc<dyn EventStore<Event = E>>
    where
        E: DomainEvent + Send + Sync + Clone + 'static,
    {
        let s = MemoryEventStore::new();
        Arc::new(s)
    }

    /// Construct an in-memory [`Projection`] that folds events into a read model.
    ///
    /// `initial` seeds the read model; `reducer` is invoked once per applied
    /// event to update it.  The returned projection takes `&mut self` on
    /// [`apply`](Projection::apply) — wrap it in a `Mutex` for shared ownership.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut p = Domain.new_in_memory_projection::<OrderEvent, u64, _>(
    ///     0,
    ///     |total, env| *total += 1,
    /// );
    /// ```
    #[cfg(feature = "projection")]
    pub fn new_in_memory_projection<E, R, F>(
        &self,
        initial: R,
        reducer: F,
    ) -> Box<dyn Projection<Event = E, ReadModel = R>>
    where
        E: DomainEvent + Send + Sync + 'static,
        R: Send + Sync + 'static,
        F: Fn(&mut R, &E) + Send + Sync + 'static,
    {
        Box::new(MemoryProjection::new(initial, reducer))
    }

    /// Construct a fresh in-memory [`SagaStore`] for saga type `S`.
    ///
    /// Sagas are stored keyed by their [`SagaId`](Saga::SagaId).  The store
    /// returns [`SagaError`](crate::SagaError) on duplicate registration or
    /// missing lookup; dispatching the commands a saga emits remains the
    /// caller's responsibility.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut store = Domain.new_in_memory_saga_store::<OrderSaga>();
    /// store.register(order_id, OrderSaga::default())?;
    /// ```
    #[cfg(feature = "saga")]
    pub fn new_in_memory_saga_store<S>(&self) -> Box<dyn SagaStore<SagaInstance = S>>
    where
        S: Saga + 'static,
        S::SagaId: std::fmt::Display + 'static,
    {
        Box::new(MemorySagaStore::new())
    }

    /// Construct a thread-safe in-memory [`SnapshotStore`] for snapshot type `S`.
    ///
    /// Keeps the latest snapshot per aggregate.  `save` rejects snapshots at
    /// version `0` with [`SnapshotError::InvalidVersion`](crate::SnapshotError);
    /// `load` returns `Ok(None)` when no snapshot exists for an aggregate.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let store = Domain.new_in_memory_snapshot_store::<OrderSnapshot>();
    /// store.save(snapshot).await?;
    /// let restored = store.load(&order_id).await?;
    /// ```
    #[cfg(feature = "snapshot")]
    pub fn new_in_memory_snapshot_store<S>(
        &self,
    ) -> Arc<dyn SnapshotStore<AggregateId = S::AggregateId, Snap = S>>
    where
        S: Snapshot + Clone + 'static,
        S::AggregateId: std::fmt::Display + 'static,
    {
        Arc::new(MemorySnapshotStore::new())
    }

    /// Reconstitute an aggregate by replaying all events from an [`EventStore`].
    ///
    /// Returns `None` when no events exist for `aggregate_id`.
    #[cfg(feature = "event")]
    pub async fn reconstitute<A>(
        &self,
        store: &dyn EventStore<Event = A::Event>,
        aggregate_id: &str,
    ) -> Result<Option<A>, EventStoreError>
    where
        A: Aggregate,
    {
        let envelopes = store
            .load(EventStoreLoadRequest { aggregate_id })
            .await?
            .events;
        if envelopes.is_empty() {
            return Ok(None);
        }
        let mut aggregate = A::default();
        for envelope in &envelopes {
            let _ = aggregate.apply(AggregateApplyRequest {
                event: &envelope.event,
            });
        }
        Ok(Some(aggregate))
    }

    /// Construct a [`QueryBus`] that dispatches queries inline.
    #[cfg(feature = "query")]
    pub fn direct_query_bus<R: Send + 'static>(&self) -> Arc<dyn QueryBus<Result = R>> {
        let b = DirectQueryBus::<R>::new();
        Arc::new(b)
    }

    /// Validate a configuration value using its [`Validator`](crate::api::Validator) impl.
    #[cfg(feature = "validator")]
    pub fn validate_config<V: crate::api::Validator>(&self, config: &V) -> Result<(), String> {
        config
            .validate(edge_application_validator::ValidationRequest)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

impl DomainRuntime for Domain {
    /// Construct a [`CommandBus`] that dispatches commands inline.
    #[cfg(feature = "command")]
    fn direct_command_bus(
        &self,
        _req: DirectCommandBusRequest,
    ) -> Result<DirectCommandBusResponse, DomainError> {
        Ok(DirectCommandBusResponse {
            bus: Arc::new(DirectCommandBus),
        })
    }

    /// Construct an [`EventPublisher`] that discards all events silently.
    #[cfg(feature = "event")]
    fn noop_event_publisher(
        &self,
        _req: NoopEventPublisherRequest,
    ) -> Result<NoopEventPublisherResponse, DomainError> {
        Ok(NoopEventPublisherResponse {
            publisher: Arc::new(NoopEventPublisher),
        })
    }

    /// Construct an in-process broadcast-backed [`EventBus`].
    ///
    /// Backed by the tokio broadcast implementation in `edge-domain-event`.
    #[cfg(feature = "event")]
    fn in_process_event_bus(
        &self,
        req: InProcessEventBusRequest,
    ) -> Result<InProcessEventBusResponse, DomainError> {
        Ok(InProcessEventBusResponse {
            bus: Arc::new(InProcessEventBus::new(req.config.capacity)),
        })
    }

    /// Construct an [`EventBus`] that silently discards all events.
    #[cfg(feature = "event")]
    fn noop_event_bus(
        &self,
        _req: NoopEventBusRequest,
    ) -> Result<NoopEventBusResponse, DomainError> {
        Ok(NoopEventBusResponse {
            bus: Arc::new(NoopEventBus),
        })
    }
}
