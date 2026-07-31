//! Integration test: the `Command` counterpart to `auth_handler_int_test.rs`. A single
//! `Handler::execute()` call that both reads `HandlerContext` for real and, as part of the
//! same call, dispatches a real `Command` — `Handler` -> `Command`, connected and working, as
//! one observable call chain.
//!
//! `Command::execute` returns `Result<(), CommandError>` — no payload, by design (the write
//! side of CQRS; see `docs/3-design/dataflow.md` section 6's three-tier table). So unlike the
//! `Query` version, there's no return value to assert on directly — the proof that the real
//! `Command` ran is its side effect: this test asserts against a shared store the `Command`
//! itself writes into, which is the correct way to observe a write, not a read.
//!
//! (Formerly `LoginRecorderSvc` implementing `Service`, wrapped by a hand-composed
//! `LoginHandler` — `Service`/`ServiceRegistry` were removed as redundant with
//! `Handler`/`HandlerRegistry`, see issue #147. Collapsed into one `Handler` directly.)
#![allow(clippy::unwrap_used)]

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use edge_application_command::{
    Command as DomainCommand, CommandBus, CommandDispatchRequest, CommandError,
    DirectCommandBus, ExecutionRequest as CommandExecutionRequest,
};
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone)]
struct RecordLoginRequest {
    session_token: String,
}
impl edge_application_base::Request for RecordLoginRequest {}

#[derive(Debug, Clone, Copy)]
struct RecordLoginResponse {
    recorded: bool,
}
impl edge_application_base::Response for RecordLoginResponse {}

/// The actual infra unit — a write, reached only through `LoginHandler`'s own `CommandBus`.
/// Writes into `store` as its observable side effect.
struct RecordLoginCommand {
    session_token: String,
    store: Arc<Mutex<Vec<String>>>,
}

impl DomainCommand for RecordLoginCommand {
    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + '_>> {
        let token = self.session_token.clone();
        let store = Arc::clone(&self.store);
        Box::pin(async move {
            store.lock().unwrap().push(token);
            Ok(())
        })
    }
}

/// Holds its own injected `CommandBus`; genuinely reads `req.ctx` inside `execute()`.
struct LoginHandler {
    command_bus: Arc<dyn CommandBus>,
    store: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl Handler for LoginHandler {
    type Request = RecordLoginRequest;
    type Response = RecordLoginResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, RecordLoginRequest>,
    ) -> Result<RecordLoginResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "login_handler".to_string(),
                message: format!("recording login for {:?}", req.req.session_token),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        self.command_bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(RecordLoginCommand {
                    session_token: req.req.session_token,
                    store: Arc::clone(&self.store),
                }),
            })
            .await
            .map_err(|e: CommandError| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(RecordLoginResponse { recorded: true })
    }
}

fn build_handler() -> (LoginHandler, Arc<Mutex<Vec<String>>>) {
    let store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let command_bus: Arc<dyn CommandBus> = Arc::new(DirectCommandBus);
    let handler = LoginHandler {
        command_bus,
        store: Arc::clone(&store),
    };
    (handler, store)
}

fn run(handler: &LoginHandler, session_token: &str) -> Result<RecordLoginResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: RecordLoginRequest {
            session_token: session_token.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_valid_token_writes_to_store_happy() {
    let (handler, store) = build_handler();
    let response = run(&handler, "abc123").unwrap();
    assert!(response.recorded);
    assert_eq!(store.lock().unwrap().as_slice(), ["abc123"]);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_token_still_writes_recorded_entry_error() {
    let (handler, store) = build_handler();
    let response = run(&handler, "").unwrap();
    assert!(response.recorded);
    assert_eq!(store.lock().unwrap().as_slice(), [""]);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_accumulate_in_store_edge() {
    let (handler, store) = build_handler();
    run(&handler, "session-a").unwrap();
    run(&handler, "session-b").unwrap();
    run(&handler, "session-c").unwrap();
    assert_eq!(
        store.lock().unwrap().as_slice(),
        ["session-a", "session-b", "session-c"]
    );
}
