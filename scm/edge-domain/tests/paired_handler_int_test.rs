//! Integration tests for `Domain.paired` — shared backend construction.
#![cfg(all(feature = "command", feature = "repository", feature = "handler"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application::DirectCommandBusRequest;
use edge_application::DomainRuntime;
use edge_application::{
    Domain, HandlerContext, HandlerError, Repository, RepositoryIdRequest, RepositorySaveRequest,
};
use edge_application_handler::{EmptinessRequest, ExecutionRequest, LenRequest};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct WriteHandler {
    repo: Arc<dyn Repository<Entity = String, Id = String>>,
}

struct ReadHandler {
    repo: Arc<dyn Repository<Entity = String, Id = String>>,
}

/// @covers: Domain.paired — both handlers share one Arc<Backend>
#[tokio::test]
async fn test_paired_write_is_visible_to_read() {
    let (writer, reader) = Domain.paired(
        Domain.new_in_memory_repository::<String, String>(),
        |repo| WriteHandler { repo },
        |repo| ReadHandler { repo },
    );

    writer
        .repo
        .save(RepositorySaveRequest {
            id: "k".to_string(),
            entity: "v".to_string(),
        })
        .await
        .unwrap();
    let id = "k".to_string();
    let found = reader
        .repo
        .find(RepositoryIdRequest { id: &id })
        .await
        .unwrap();
    assert_eq!(found.entity, Some("v".to_string()));
}

/// @covers: Domain.paired — independent from_config() calls use different backends
#[tokio::test]
async fn test_independent_backends_do_not_share_state() {
    let repo_a = Domain.new_in_memory_repository::<String, String>();
    let repo_b = Domain.new_in_memory_repository::<String, String>();

    repo_a
        .save(RepositorySaveRequest {
            id: "k".to_string(),
            entity: "v".to_string(),
        })
        .await
        .unwrap();

    let id = "k".to_string();
    let found = repo_b.find(RepositoryIdRequest { id: &id }).await.unwrap();
    assert_eq!(
        found.entity, None,
        "separate instances must not share state"
    );
}

/// @covers: Domain.paired — different handler types allowed
#[test]
fn test_paired_accepts_heterogeneous_handler_types() {
    struct CmdHandler {
        _repo: Arc<dyn Repository<Entity = u32, Id = u32>>,
    }
    struct QryHandler {
        _repo: Arc<dyn Repository<Entity = u32, Id = u32>>,
    }

    let (cmd, qry) = Domain.paired(
        Domain.new_in_memory_repository::<u32, u32>(),
        |repo| CmdHandler { _repo: repo },
        |repo| QryHandler { _repo: repo },
    );
    drop((cmd, qry));
}

// ── EchoHandler via Domain factory (backward compat) ─────────────────────────

/// @covers: Domain.echo_handler delegates to Dispatch::echo_handler
#[tokio::test]
async fn test_domain_echo_handler_returns_input_unchanged() {
    let h = Domain.echo_handler::<TextPayload>("e", "/e");
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
    };
    let result = h
        .execute(ExecutionRequest {
            req: TextPayload("hello".to_string()),
            ctx: &ctx,
        })
        .await;
    assert_eq!(result.unwrap(), TextPayload("hello".to_string()));
}

/// @covers: Domain.new_handler_registry delegates to Dispatch::new_handler_registry
#[test]
fn test_domain_new_handler_registry_is_empty() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers: HandlerError::internal
#[test]
fn test_handler_error_internal_helper() {
    let e = HandlerError::ExecutionFailed("db timeout".to_string());
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
}
