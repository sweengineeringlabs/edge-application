//! Integration test: `GreeterRegistrationHandler::execute()` genuinely reads `HandlerContext` for
//! real observability and, as part of the same call, registers through a real,
//! constructor-injected `Registry` — `Handler` -> `Registry`, connected and working, as one
//! observable call chain. Assertions verify the entry landed in the shared registry, and that
//! the registry's own duplicate-id validation propagates as a genuine `HandlerError`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_registry::{
    MemoryRegistry, Registry, RegistryLookupRequest, TryRegisterRequest,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

trait Greeter: Send + Sync {
    fn greet(&self, name: &str) -> String;
}

struct EnglishGreeter;
impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}

struct SpanishGreeter;
impl Greeter for SpanishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("¡Hola, {name}!")
    }
}

#[derive(Debug, Clone)]
struct GreeterRegistrationRequest {
    id: String,
    language: String,
}
impl edge_application_base::Request for GreeterRegistrationRequest {}

#[derive(Debug, Clone, Copy)]
struct GreeterRegistrationResponse {
    registered: bool,
}
impl edge_application_base::Response for GreeterRegistrationResponse {}

/// Holds its own injected `Registry`; genuinely reads `req.ctx` inside `execute()`.
struct GreeterRegistrationHandler {
    registry: Arc<dyn Registry<Value = dyn Greeter>>,
}

#[async_trait]
impl Handler for GreeterRegistrationHandler {
    type Request = GreeterRegistrationRequest;
    type Response = GreeterRegistrationResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, GreeterRegistrationRequest>,
    ) -> Result<GreeterRegistrationResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "greeter_registration_handler".to_string(),
                message: format!("registering {:?} greeter under {:?}", req.req.language, req.req.id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        let greeter: Arc<dyn Greeter> = match req.req.language.as_str() {
            "es" => Arc::new(SpanishGreeter),
            _ => Arc::new(EnglishGreeter),
        };
        self.registry
            .try_register(TryRegisterRequest {
                id: req.req.id,
                entry: greeter,
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(GreeterRegistrationResponse { registered: true })
    }
}

fn build_handler() -> (GreeterRegistrationHandler, Arc<dyn Registry<Value = dyn Greeter>>) {
    let registry: Arc<dyn Registry<Value = dyn Greeter>> =
        Arc::new(MemoryRegistry::<dyn Greeter>::new());
    let handler = GreeterRegistrationHandler {
        registry: Arc::clone(&registry),
    };
    (handler, registry)
}

fn run(
    handler: &GreeterRegistrationHandler,
    id: &str,
    language: &str,
) -> Result<GreeterRegistrationResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: GreeterRegistrationRequest {
            id: id.to_string(),
            language: language.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_new_id_persists_in_registry_happy() {
    let (handler, registry) = build_handler();
    let response = run(&handler, "es-greeter", "es").unwrap();
    assert!(response.registered);

    let resolved = registry
        .get(RegistryLookupRequest {
            id: "es-greeter".to_string(),
        })
        .unwrap()
        .entry;
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().greet("Ada"), "¡Hola, Ada!");
}

/// @covers: Handler::execute
#[test]
fn test_execute_duplicate_id_returns_execution_failed_error() {
    let (handler, _registry) = build_handler();
    run(&handler, "greeter-1", "en").unwrap();
    let err = run(&handler, "greeter-1", "es").unwrap_err();
    assert!(matches!(err, HandlerError::ExecutionFailed(_)));
}

/// @covers: Handler::execute
#[test]
fn test_execute_distinct_ids_both_registered_edge() {
    let (handler, registry) = build_handler();
    run(&handler, "greeter-en", "en").unwrap();
    run(&handler, "greeter-es", "es").unwrap();

    assert_eq!(registry.len(edge_application_registry::LenRequest).unwrap().count, 2);
}
