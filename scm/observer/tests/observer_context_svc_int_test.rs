#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, DrainRequest, GaugeLookupRequest, GaugeSetRequest, IncrementRequest,
    LogEmitRequest, MetricsRequest, ObserverContext, SpanFinishRequest,
    SpanStartRequest, StdObserveFactory, TracerRequest, OBSERVE_CONTEXT_SVC,
};

// ── factory method ────────────────────────────────────────────────────────────

#[test]
fn test_noop_observer_context_svc_builds_usable_context_happy() {
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    ctx.tracer(TracerRequest)
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
    ctx.drain(DrainRequest)
        .unwrap()
        .drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
    ctx.metrics(MetricsRequest)
        .unwrap()
        .metrics
        .counter(CounterLookupRequest {
            name: "c".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 1 })
        .unwrap();
}

#[test]
fn test_noop_observer_context_svc_tracer_no_panic_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let tracer = ctx.tracer(TracerRequest).unwrap().tracer;
    for i in 0..3 {
        let span = tracer
            .start_span(SpanStartRequest {
                handler_id: format!("h{i}"),
                operation: "op".to_string(),
            })
            .unwrap()
            .span;
        span.finish(SpanFinishRequest).unwrap();
        assert_eq!(std::mem::size_of_val(&*span), 0, "noop span is ZST");
    }
}

// ── service key ───────────────────────────────────────────────────────────────

#[test]
fn test_observe_context_svc_key_namespaced_happy() {
    assert!(OBSERVE_CONTEXT_SVC.starts_with("edge."));
}

#[test]
fn test_noop_observer_context_svc_empty_span_ids_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let span = ctx
        .tracer(TracerRequest)
        .unwrap()
        .tracer
        .start_span(SpanStartRequest {
            handler_id: "".to_string(),
            operation: "".to_string(),
        })
        .unwrap()
        .span;
    span.finish(SpanFinishRequest).unwrap();
    let drain = ctx.drain(DrainRequest).unwrap().drain;
    drain
        .emit(LogEmitRequest {
            level: String::new(),
            handler_id: String::new(),
            message: String::new(),
        })
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*span), 0, "noop span is ZST");
}

// ── noop_arc_observe_context ──────────────────────────────────────────────────

#[test]
fn test_noop_arc_observe_context_svc_builds_usable_context_happy() {
    use std::sync::Arc;
    let ctx: Arc<dyn ObserverContext> = StdObserveFactory::noop_arc_observe_context();
    ctx.tracer(TracerRequest)
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
    ctx.drain(DrainRequest)
        .unwrap()
        .drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "h".to_string(),
            message: "msg".to_string(),
        })
        .unwrap();
    ctx.metrics(MetricsRequest)
        .unwrap()
        .metrics
        .counter(CounterLookupRequest {
            name: "c".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 1 })
        .unwrap();
}

#[test]
fn test_noop_arc_observe_context_svc_empty_inputs_no_panic_error() {
    use std::sync::Arc;
    let ctx: Arc<dyn ObserverContext> = StdObserveFactory::noop_arc_observe_context();
    ctx.tracer(TracerRequest)
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
    ctx.drain(DrainRequest)
        .unwrap()
        .drain
        .emit(LogEmitRequest {
            level: String::new(),
            handler_id: String::new(),
            message: String::new(),
        })
        .unwrap();
}

#[test]
fn test_noop_arc_observe_context_svc_arc_clone_shares_same_ptr_edge() {
    use std::sync::Arc;
    let ctx: Arc<dyn ObserverContext> = StdObserveFactory::noop_arc_observe_context();
    let ctx2 = Arc::clone(&ctx);
    assert!(Arc::ptr_eq(&ctx, &ctx2));
}

// ── independence ──────────────────────────────────────────────────────────────

#[test]
fn test_noop_observer_context_svc_multiple_calls_independent_edge() {
    let a = StdObserveFactory::noop_observer_context();
    let b = StdObserveFactory::noop_observer_context();
    let gauge_a = a
        .metrics(MetricsRequest)
        .unwrap()
        .metrics
        .gauge(GaugeLookupRequest {
            name: "g".to_string(),
        })
        .unwrap()
        .gauge;
    let gauge_b = b
        .metrics(MetricsRequest)
        .unwrap()
        .metrics
        .gauge(GaugeLookupRequest {
            name: "g".to_string(),
        })
        .unwrap()
        .gauge;
    gauge_a.set(GaugeSetRequest { value: 1.0 }).unwrap();
    gauge_b.set(GaugeSetRequest { value: 2.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*gauge_a), 0, "contexts are independent");
}
