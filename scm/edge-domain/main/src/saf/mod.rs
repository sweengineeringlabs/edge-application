//! SAF layer — domain public facade.

#[cfg(test)]
mod tests;

mod aggregate_svc;
mod clock;
mod command;
mod domain;
mod entity_svc;
mod event;
mod handler;
mod policy;
mod policy_svc;
mod projection_svc;
mod query;
mod queryable_repository_svc;
mod repository;
mod repository_svc;
mod saga;
mod snapshot;
mod spec_svc;
mod spi;
mod validator_svc;
mod value;
mod value_object_svc;

// ── sub-module re-exports (all api/ exports flow through _svc.rs inside each subdir) ─
pub use self::clock::*;
pub use self::command::*;
pub use self::domain::*;
pub use self::event::*;
pub use self::handler::*;
pub use self::policy::*;
pub use self::query::*;
pub use self::repository::*;
pub use self::saga::*;
pub use self::snapshot::*;
pub use self::spi::*;
pub use self::value::*;

// ── top-level _svc.rs re-exports ─────────────────────────────────────────────
pub use self::aggregate_svc::*;
pub use self::entity_svc::*;
pub use self::policy_svc::*;
pub use self::projection_svc::*;
pub use self::queryable_repository_svc::*;
pub use self::repository_svc::*;
pub use self::spec_svc::*;
pub use self::validator_svc::*;
pub use self::value_object_svc::*;

// ── entity (sub-crate when feature enabled) ───────────────────────────────────
#[cfg(feature = "entity")]
pub use edge_application_entity::Entity;

// ── valueobject (sub-crate when feature enabled) ─────────────────────────────
#[cfg(feature = "valueobject")]
pub use edge_application_valueobject::NonEmptyString;
#[cfg(feature = "valueobject")]
pub use edge_application_valueobject::ValueObject;
#[cfg(feature = "valueobject")]
pub use edge_application_valueobject::ValueObjectError;

// ── clock (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "clock")]
pub use edge_application_clock::Clock;
#[cfg(feature = "clock")]
pub use edge_application_clock::FixedClock;
#[cfg(feature = "clock")]
pub use edge_application_clock::SystemClock;

// ── validator (sub-crate when feature enabled; re-exported publicly via api::*) ─
#[cfg(feature = "validator")]
pub(crate) use edge_application_validator::Validator;
#[cfg(feature = "validator")]
pub use edge_application_validator::ValidatorError;

// ── policy (sub-crate when feature enabled) ───────────────────────────────────
#[cfg(feature = "policy")]
pub use edge_application_policy::CompositePolicy;
#[cfg(feature = "policy")]
pub use edge_application_policy::Policy;
#[cfg(feature = "policy")]
pub use edge_application_policy::PolicyError;
#[cfg(feature = "policy")]
pub use edge_application_policy::PolicyEvaluateRequest;
#[cfg(feature = "policy")]
pub use edge_application_policy::PolicyNameRequest;
#[cfg(feature = "policy")]
pub use edge_application_policy::PolicyNameResponse;

// ── command (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "command")]
pub use edge_application_command::Command;
#[cfg(feature = "command")]
pub use edge_application_command::CommandError;

// ── query (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_application_query::Query;
#[cfg(feature = "query")]
pub use edge_application_query::QueryError;

// ── snapshot (sub-crate when feature enabled) ─────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotAggregateIdRequest;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotAggregateIdResponse;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotError;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotLoadRequest;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotLoadResponse;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotSaveRequest;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotVersionRequest;
#[cfg(feature = "snapshot")]
pub use edge_application_snapshot::SnapshotVersionResponse;

// ── repository (sub-crate when feature enabled) ───────────────────────────────
#[cfg(feature = "repository")]
pub use edge_application_repository::Page;
#[cfg(feature = "repository")]
pub use edge_application_repository::RepositoryError;
#[cfg(feature = "repository")]
pub use edge_application_repository::Spec;
#[cfg(feature = "repository")]
pub use edge_application_repository::QUERYABLE_REPOSITORY_SVC;
#[cfg(feature = "repository")]
pub use edge_application_repository::QUERYABLE_REPOSITORY_SVC_FACTORY;
#[cfg(feature = "repository")]
pub use edge_application_repository::REPOSITORY_SVC;
#[cfg(feature = "repository")]
pub use edge_application_repository::REPOSITORY_SVC_FACTORY;
#[cfg(feature = "repository")]
pub use edge_application_repository::SPEC_SVC;
#[cfg(feature = "repository")]
pub use edge_application_repository::SPEC_SVC_FACTORY;

// ── handler (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_application_handler::HandlerContext;
#[cfg(feature = "handler")]
pub use edge_application_handler::HandlerError;

// ── event (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_application_event::ClosedEventSource;
#[cfg(feature = "event")]
pub use edge_application_event::EventEnvelope;
#[cfg(feature = "event")]
pub use edge_application_event::EventSource;
#[cfg(feature = "event")]
pub use edge_application_event::ExpectedVersion;

// ── pipeline (sub-crate when feature enabled) ────────────────────────────────
#[cfg(feature = "pipeline")]
pub use edge_pipeline::ContextMutationRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::Pipeline;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineBuilder;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineConfig;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineEmptinessRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineError;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::PipelineSvc;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::Step;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepCountRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepNameRequest;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepNameResponse;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepRegistry as PipelineStepRegistry;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::StepRegistrySvc as PipelineStepRegistrySvc;
#[cfg(feature = "pipeline")]
pub(crate) use edge_pipeline::Validator as PipelineValidator;
#[cfg(feature = "pipeline")]
pub use edge_pipeline::ValidatorSvc as PipelineValidatorSvc;

// ── projection (sub-crate when feature enabled) ───────────────────────────────
#[cfg(feature = "projection")]
pub use edge_application_projection::ProjectionError;
#[cfg(feature = "projection")]
pub use edge_application_projection::TryDrainResponse;

// ── saga (sub-crate when feature enabled) ─────────────────────────────────────
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaError;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaGetRequest;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaGetResponse;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaHandleRequest;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaHandleResponse;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaIsCompleteRequest;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaIsCompleteResponse;
#[cfg(feature = "saga")]
pub use edge_application_saga::SagaRegisterRequest;

// ── registry (opt-in; NOT in default features) ────────────────────────────────
#[cfg(feature = "registry")]
pub use edge_application_registry::MemoryRegistry;
#[cfg(feature = "registry")]
pub use edge_application_registry::Registry;
#[cfg(feature = "registry")]
pub use edge_application_registry::RegistryError;

// ── lifecycle (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::Lifecycle;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::LifecycleError;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::LifecycleIsInRequest;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::LifecycleIsInResponse;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::LifecycleStateRequest;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::LifecycleStateResponse;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::LifecycleTransitionRequest;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::ManagedLifecycle;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::PermissivePolicy;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::TransitionAllowedRequest;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::TransitionAllowedResponse;
#[cfg(feature = "lifecycle")]
pub use edge_application_lifecycle::TransitionPolicy;

// ── security ──────────────────────────────────────────────────────────────────
#[cfg(feature = "security")]
pub use edge_security_authn::AuthenticateRequest;
#[cfg(feature = "security")]
pub use edge_security_authn::Authenticator;
#[cfg(feature = "security")]
pub use edge_security_authn::AuthnContext;
#[cfg(feature = "security")]
pub use edge_security_authn::AuthnError;
#[cfg(feature = "security")]
pub use edge_security_authz::AuthorizeRequest;
#[cfg(feature = "security")]
pub use edge_security_authz::Authorizer;
#[cfg(feature = "security")]
pub use edge_security_authz::AuthzContext;
#[cfg(feature = "security")]
pub use edge_security_authz::AuthzError;
#[cfg(feature = "security")]
pub use edge_security_runtime::AnonymousPrincipal;
#[cfg(feature = "security")]
pub use edge_security_runtime::Principal;
#[cfg(feature = "security")]
pub use edge_security_runtime::SecurityContext;
#[cfg(feature = "security")]
pub use edge_security_runtime::SecurityError;

// ── observer (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "observer")]
pub use edge_application_observer::ObserveBootstrap;
#[cfg(feature = "observer")]
pub use edge_application_observer::ObserverContext;
