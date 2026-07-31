#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, DrainRequest, GaugeLookupRequest, GaugeSetRequest,
    HistogramLookupRequest, HistogramRecordRequest, IncrementRequest, LogEmitRequest,
    MetricsRequest, ObserverContext, SpanFinishRequest, SpanStartRequest, StdObserveFactory,
    TracerRequest,
};
use std::sync::Arc;

// ── tracer ───────────────────────────────────────────────────────────────────

/// @covers: ObserverContext::tracer
#[test]
fn test_tracer_returns_handler_tracer_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let t = ctx.tracer(TracerRequest).unwrap().tracer;
    t.start_span(SpanStartRequest {
        handler_id: "handler_a".to_string(),
        operation: "execute".to_string(),
    })
    .unwrap()
    .span
    .finish(SpanFinishRequest)
    .unwrap();
    assert_eq!(std::mem::size_of_val(t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_tracer_empty_ids_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let t = ctx.tracer(TracerRequest).unwrap().tracer;
    t.start_span(SpanStartRequest {
        handler_id: "".to_string(),
        operation: "".to_string(),
    })
    .unwrap()
    .span
    .finish(SpanFinishRequest)
    .unwrap();
    assert_eq!(std::mem::size_of_val(t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_tracer_multiple_spans_edge() {
    let ctx = StdObserveFactory::noop_observer_context();
    let t = ctx.tracer(TracerRequest).unwrap().tracer;
    for i in 0..5 {
        t.start_span(SpanStartRequest {
            handler_id: format!("h{i}"),
            operation: "op".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    }
    assert_eq!(std::mem::size_of_val(t), 0, "noop handler tracer is ZST");
}

// ── drain ────────────────────────────────────────────────────────────────────

/// @covers: ObserverContext::drain
#[test]
fn test_drain_emits_log_record_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let d = ctx.drain(DrainRequest).unwrap().drain;
    d.emit(LogEmitRequest {
        level: "INFO".to_string(),
        handler_id: "handler_a".to_string(),
        message: "started".to_string(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(d), 0, "noop log drain is ZST");
}

#[test]
fn test_drain_empty_fields_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let d = ctx.drain(DrainRequest).unwrap().drain;
    d.emit(LogEmitRequest {
        level: String::new(),
        handler_id: String::new(),
        message: String::new(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(d), 0, "noop log drain is ZST");
}

#[test]
fn test_drain_repeated_emit_no_accumulation_edge() {
    let ctx = StdObserveFactory::noop_observer_context();
    let d = ctx.drain(DrainRequest).unwrap().drain;
    for i in 0..10 {
        d.emit(LogEmitRequest {
            level: "DEBUG".to_string(),
            handler_id: "h".to_string(),
            message: format!("msg {i}"),
        })
        .unwrap();
    }
    assert_eq!(std::mem::size_of_val(d), 0, "noop log drain is ZST");
}

// ── metrics ──────────────────────────────────────────────────────────────────

/// @covers: ObserverContext::metrics
#[test]
fn test_metrics_counter_increments_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let r = ctx.metrics(MetricsRequest).unwrap().metrics;
    let c = r
        .counter(CounterLookupRequest {
            name: "req.count".to_string(),
        })
        .unwrap()
        .counter;
    c.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}

#[test]
fn test_metrics_empty_name_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let r = ctx.metrics(MetricsRequest).unwrap().metrics;
    let c = r
        .counter(CounterLookupRequest {
            name: "".to_string(),
        })
        .unwrap()
        .counter;
    let g = r
        .gauge(GaugeLookupRequest {
            name: "".to_string(),
        })
        .unwrap()
        .gauge;
    c.increment(IncrementRequest { delta: 0 }).unwrap();
    g.set(GaugeSetRequest { value: 0.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_metrics_all_instrument_types_edge() {
    let ctx = StdObserveFactory::noop_observer_context();
    let r = ctx.metrics(MetricsRequest).unwrap().metrics;
    let c = r
        .counter(CounterLookupRequest { name: "c".to_string() })
        .unwrap()
        .counter;
    let g = r
        .gauge(GaugeLookupRequest { name: "g".to_string() })
        .unwrap()
        .gauge;
    let h = r
        .histogram(HistogramLookupRequest { name: "h".to_string() })
        .unwrap()
        .histogram;
    c.increment(IncrementRequest { delta: 1 }).unwrap();
    g.set(GaugeSetRequest { value: 1.0 }).unwrap();
    h.record(HistogramRecordRequest { value: 1.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
}

// ── object safety ─────────────────────────────────────────────────────────────

#[test]
fn test_observe_context_is_object_safe_error() {
    fn _takes(_: Arc<dyn ObserverContext>) {}
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    let a: Arc<dyn ObserverContext> = Arc::from(ctx);
    _takes(a.clone());
    assert!(Arc::strong_count(&a) > 0, "Arc should be valid after passing to function");
}

// ── multiple instances ────────────────────────────────────────────────────────

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_noop_observer_context_multiple_instances_independent_edge() {
    let a: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    let b: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    let ta = a.tracer(TracerRequest).unwrap().tracer;
    let tb = b.tracer(TracerRequest).unwrap().tracer;
    ta.start_span(SpanStartRequest {
        handler_id: "a".to_string(),
        operation: "op".to_string(),
    })
    .unwrap()
    .span
    .finish(SpanFinishRequest)
    .unwrap();
    tb.start_span(SpanStartRequest {
        handler_id: "b".to_string(),
        operation: "op".to_string(),
    })
    .unwrap()
    .span
    .finish(SpanFinishRequest)
    .unwrap();
    assert_eq!(std::mem::size_of_val(ta), 0, "noop handler tracer is ZST");
    assert_eq!(std::mem::size_of_val(tb), 0, "noop handler tracer is ZST");
}
