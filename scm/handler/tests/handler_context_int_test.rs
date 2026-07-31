//! Integration tests — `HandlerContext` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    CommandDispatchRequest, HandlerContext, SecurityPrincipal, SpanFinishRequest,
    SpanStartRequest, TracerRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

/// Identity pointer for a `&dyn SecurityPrincipal`, so tests can assert `HandlerContext`
/// carries the same principal through without exposing the concrete `SecurityContext` type.
fn principal_ptr(p: &dyn SecurityPrincipal) -> *const () {
    p as *const dyn SecurityPrincipal as *const ()
}

fn security_ptr(security: &SecurityContext) -> *const () {
    security as *const SecurityContext as *const ()
}

/// @covers: HandlerContext — constructs with unauthenticated security and direct bus
#[test]
fn test_handler_context_constructs_with_unauthenticated_security_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    // Verify the context holds a reference to the same security object
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
}

/// @covers: HandlerContext::commands — dispatch on direct bus succeeds
#[test]
fn test_handler_context_commands_field_is_accessible_error() {
    use edge_application_command::NoopCommand;
    use futures::executor::block_on;

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = block_on(ctx.commands.dispatch(CommandDispatchRequest {
        command: Box::new(NoopCommand),
    }));
    assert!(result.is_ok());
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
}

/// @covers: HandlerContext — Copy semantics allow multiple uses without move
#[test]
fn test_handler_context_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let ctx2 = ctx;
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
    assert!(principal_ptr(ctx2.security) == security_ptr(&security));
}

/// @covers: HandlerContext::observer — returns bound ObserverContext
#[test]
fn test_observer_returns_bound_observe_context_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    ctx.observer
        .tracer(TracerRequest)
        .unwrap()
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "h".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
}

/// @covers: HandlerContext::observer — tracer usable across multiple spans
#[test]
fn test_observer_tracer_usable_after_construction_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    for i in 0..3 {
        ctx.observer
            .tracer(TracerRequest)
            .unwrap()
            .tracer
            .start_span(SpanStartRequest {
                handler_id: format!("span_{i}"),
                operation: "op".to_string(),
            })
            .unwrap()
            .span
            .finish(SpanFinishRequest)
            .unwrap();
    }
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
}

/// @covers: HandlerContext::observer — empty span ids do not panic
#[test]
fn test_observer_empty_span_ids_no_panic_error() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    ctx.observer
        .tracer(TracerRequest)
        .unwrap()
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "".to_string(),
            operation: "".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
}

/// @covers: HandlerContext — Copy semantics preserved with observer field
#[test]
fn test_handler_context_with_observer_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let ctx2 = ctx;
    ctx.observer
        .tracer(TracerRequest)
        .unwrap()
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "ctx1".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    ctx2.observer
        .tracer(TracerRequest)
        .unwrap()
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "ctx2".to_string(),
            operation: "op".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    assert!(principal_ptr(ctx.security) == security_ptr(&security));
}
