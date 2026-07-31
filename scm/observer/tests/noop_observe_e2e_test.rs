#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    BootstrapNameRequest, Counter, CounterLookupRequest, Gauge, GaugeLookupRequest,
    GaugeSetRequest, HandlerTracer, Histogram, HistogramLookupRequest, HistogramRecordRequest,
    IncrementRequest, LogDrain, LogEmitRequest, MetricRegistry, MetricsRequest, NoopObserve,
    ObserverContext, Span, SpanAnnotationRequest, SpanFinishRequest, SpanStartRequest,
    StdObserveFactory, TracerRequest,
};

// --- build_noop_counter ---

#[test]
fn test_build_noop_counter_returns_usable_counter_happy() {
    let c = StdObserveFactory::build_noop_counter();
    c.increment(IncrementRequest { delta: 1 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}

#[test]
fn test_build_noop_counter_max_delta_no_panic_error() {
    let c = StdObserveFactory::build_noop_counter();
    c.increment(IncrementRequest { delta: u64::MAX }).unwrap();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}

#[test]
fn test_build_noop_counter_multiple_instances_independent_edge() {
    let a = StdObserveFactory::build_noop_counter();
    let b = StdObserveFactory::build_noop_counter();
    a.increment(IncrementRequest { delta: 1 }).unwrap();
    b.increment(IncrementRequest { delta: 2 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop counter is ZST");
}

// --- build_noop_gauge ---

#[test]
fn test_build_noop_gauge_returns_usable_gauge_happy() {
    let g = StdObserveFactory::build_noop_gauge();
    g.set(GaugeSetRequest { value: 42.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_build_noop_gauge_negative_value_no_panic_error() {
    let g = StdObserveFactory::build_noop_gauge();
    g.set(GaugeSetRequest { value: -1.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_build_noop_gauge_multiple_instances_independent_edge() {
    let a = StdObserveFactory::build_noop_gauge();
    let b = StdObserveFactory::build_noop_gauge();
    a.set(GaugeSetRequest { value: 0.0 }).unwrap();
    b.set(GaugeSetRequest { value: f64::MAX }).unwrap();
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop gauge is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop gauge is ZST");
}

// --- build_noop_histogram ---

#[test]
fn test_build_noop_histogram_returns_usable_histogram_happy() {
    let h = StdObserveFactory::build_noop_histogram();
    h.record(HistogramRecordRequest { value: 25.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
}

#[test]
fn test_build_noop_histogram_zero_value_no_panic_error() {
    let h = StdObserveFactory::build_noop_histogram();
    h.record(HistogramRecordRequest { value: 0.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
}

#[test]
fn test_build_noop_histogram_multiple_instances_independent_edge() {
    let a = StdObserveFactory::build_noop_histogram();
    let b = StdObserveFactory::build_noop_histogram();
    a.record(HistogramRecordRequest { value: 1.0 }).unwrap();
    b.record(HistogramRecordRequest { value: 2.0 }).unwrap();
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop histogram is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop histogram is ZST");
}

// --- build_noop_span ---

#[test]
fn test_build_noop_span_returns_usable_span_happy() {
    let s = StdObserveFactory::build_noop_span();
    s.record(SpanAnnotationRequest {
        key: "k".to_string(),
        value: "v".to_string(),
    })
    .unwrap();
    s.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*s), 0, "noop span is ZST");
}

#[test]
fn test_build_noop_span_empty_key_value_no_panic_error() {
    let s = StdObserveFactory::build_noop_span();
    s.record(SpanAnnotationRequest {
        key: "".to_string(),
        value: "".to_string(),
    })
    .unwrap();
    s.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*s), 0, "noop span is ZST");
}

#[test]
fn test_build_noop_span_multiple_finish_calls_edge() {
    let a = StdObserveFactory::build_noop_span();
    let b = StdObserveFactory::build_noop_span();
    a.finish(SpanFinishRequest).unwrap();
    b.finish(SpanFinishRequest).unwrap();
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop span is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop span is ZST");
}

// --- build_noop_handler_tracer ---

#[test]
fn test_build_noop_handler_tracer_returns_usable_tracer_happy() {
    let t = StdObserveFactory::build_noop_handler_tracer();
    t.start_span(SpanStartRequest {
        handler_id: "h".to_string(),
        operation: "op".to_string(),
    })
    .unwrap()
    .span
    .finish(SpanFinishRequest)
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_build_noop_handler_tracer_empty_ids_no_panic_error() {
    let t = StdObserveFactory::build_noop_handler_tracer();
    t.start_span(SpanStartRequest {
        handler_id: "".to_string(),
        operation: "".to_string(),
    })
    .unwrap()
    .span
    .finish(SpanFinishRequest)
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_build_noop_handler_tracer_multiple_spans_edge() {
    let t = StdObserveFactory::build_noop_handler_tracer();
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
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

// --- build_noop_log_drain ---

#[test]
fn test_build_noop_log_drain_returns_usable_drain_happy() {
    let d = StdObserveFactory::build_noop_log_drain();
    d.emit(LogEmitRequest {
        level: "INFO".to_string(),
        handler_id: "h".to_string(),
        message: "msg".to_string(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

#[test]
fn test_build_noop_log_drain_empty_fields_no_panic_error() {
    let d = StdObserveFactory::build_noop_log_drain();
    d.emit(LogEmitRequest {
        level: String::new(),
        handler_id: String::new(),
        message: String::new(),
    })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

#[test]
fn test_build_noop_log_drain_multiple_emits_no_accumulation_edge() {
    let d = StdObserveFactory::build_noop_log_drain();
    for i in 0..10 {
        d.emit(LogEmitRequest {
            level: "DEBUG".to_string(),
            handler_id: "h".to_string(),
            message: format!("{i}"),
        })
        .unwrap();
    }
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

// --- build_noop_metric_registry ---

#[test]
fn test_build_noop_metric_registry_returns_usable_registry_happy() {
    let r = StdObserveFactory::build_noop_metric_registry();
    r.counter(CounterLookupRequest {
        name: "req".to_string(),
    })
    .unwrap()
    .counter
    .increment(IncrementRequest { delta: 1 })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*r), 0, "noop metric registry is ZST");
}

#[test]
fn test_build_noop_metric_registry_empty_name_no_panic_error() {
    let r = StdObserveFactory::build_noop_metric_registry();
    r.counter(CounterLookupRequest {
        name: "".to_string(),
    })
    .unwrap()
    .counter
    .increment(IncrementRequest { delta: 0 })
    .unwrap();
    r.gauge(GaugeLookupRequest {
        name: "".to_string(),
    })
    .unwrap()
    .gauge
    .set(GaugeSetRequest { value: 0.0 })
    .unwrap();
    r.histogram(HistogramLookupRequest {
        name: "".to_string(),
    })
    .unwrap()
    .histogram
    .record(HistogramRecordRequest { value: 0.0 })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*r), 0, "noop metric registry is ZST");
}

#[test]
fn test_build_noop_metric_registry_all_instruments_edge() {
    let r = StdObserveFactory::build_noop_metric_registry();
    r.counter(CounterLookupRequest {
        name: "c".to_string(),
    })
    .unwrap()
    .counter
    .increment(IncrementRequest { delta: 1 })
    .unwrap();
    r.gauge(GaugeLookupRequest {
        name: "g".to_string(),
    })
    .unwrap()
    .gauge
    .set(GaugeSetRequest { value: 1.0 })
    .unwrap();
    r.histogram(HistogramLookupRequest {
        name: "h".to_string(),
    })
    .unwrap()
    .histogram
    .record(HistogramRecordRequest { value: 1.0 })
    .unwrap();
    assert_eq!(std::mem::size_of_val(&*r), 0, "noop metric registry is ZST");
}

// --- trait object safety / return types ---

#[test]
fn test_noop_observe_methods_return_correct_dyn_types_happy() {
    fn _takes_counter(_: Box<dyn Counter>) {}
    fn _takes_gauge(_: Box<dyn Gauge>) {}
    fn _takes_histogram(_: Box<dyn Histogram>) {}
    fn _takes_span(_: Box<dyn Span>) {}
    fn _takes_tracer(_: Box<dyn HandlerTracer>) {}
    fn _takes_drain(_: Box<dyn LogDrain>) {}
    fn _takes_registry(_: Box<dyn MetricRegistry>) {}

    let c = StdObserveFactory::build_noop_counter();
    let g = StdObserveFactory::build_noop_gauge();
    let h = StdObserveFactory::build_noop_histogram();
    let s = StdObserveFactory::build_noop_span();
    let t = StdObserveFactory::build_noop_handler_tracer();
    let d = StdObserveFactory::build_noop_log_drain();
    let r = StdObserveFactory::build_noop_metric_registry();

    _takes_counter(c);
    _takes_gauge(g);
    _takes_histogram(h);
    _takes_span(s);
    _takes_tracer(t);
    _takes_drain(d);
    _takes_registry(r);

    // Trait object conversions work correctly — reaching this point without a
    // type error is the assertion.
}

// --- noop_name ---

/// @covers: NoopObserve::noop_name
#[test]
fn test_noop_name_returns_nonempty_identifier_happy() {
    let name = StdObserveFactory
        .noop_name(BootstrapNameRequest)
        .unwrap()
        .name;
    assert!(
        !name.is_empty(),
        "noop_name must return a non-empty identifier"
    );
}

/// @covers: NoopObserve::noop_name
#[test]
fn test_noop_name_default_is_stable_error() {
    // Calling on a second instance must return the same value (no state leakage).
    let first = StdObserveFactory
        .noop_name(BootstrapNameRequest)
        .unwrap()
        .name;
    let second = StdObserveFactory
        .noop_name(BootstrapNameRequest)
        .unwrap()
        .name;
    assert_eq!(first, second);
    assert!(!first.is_empty(), "noop_name should not be empty");
}

/// @covers: NoopObserve::noop_name
#[test]
fn test_noop_name_callable_via_trait_object_edge() {
    let f: &dyn NoopObserve = &StdObserveFactory;
    let name = f.noop_name(BootstrapNameRequest).unwrap().name;
    assert!(
        !name.is_empty(),
        "noop_name via trait object should not be empty"
    );
}

// --- build_noop_observer_context ---

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_build_noop_observer_context_returns_usable_context_happy() {
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
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
    assert!(
        std::mem::size_of_val(&*ctx) > 0,
        "noop observer context is heap-backed (holds boxed tracer/drain/metrics)"
    );
}

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_build_noop_observer_context_empty_ids_no_panic_error() {
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
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
    assert!(
        std::mem::size_of_val(&*ctx) > 0,
        "noop observer context is heap-backed (holds boxed tracer/drain/metrics)"
    );
}

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_build_noop_observer_context_multiple_instances_independent_edge() {
    let a: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
    let b: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
    a.metrics(MetricsRequest)
        .unwrap()
        .metrics
        .counter(CounterLookupRequest {
            name: "a".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 1 })
        .unwrap();
    b.metrics(MetricsRequest)
        .unwrap()
        .metrics
        .counter(CounterLookupRequest {
            name: "b".to_string(),
        })
        .unwrap()
        .counter
        .increment(IncrementRequest { delta: 2 })
        .unwrap();
    let a_ptr = &*a as *const dyn ObserverContext;
    let b_ptr = &*b as *const dyn ObserverContext;
    assert_ne!(
        a_ptr, b_ptr,
        "successive calls should produce independent instances"
    );
}
