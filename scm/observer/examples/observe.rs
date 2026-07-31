//! End-to-end usage example for `edge-domain-observe`.
//!
//! Demonstrates the full observability triplet using noop implementations —
//! the same API contract that production SDK adapters satisfy.
#![allow(clippy::expect_used)]

use edge_application_observer::{
    CounterLookupRequest, GaugeLookupRequest, GaugeSetRequest, HandlerTracerBuildRequest,
    HistogramLookupRequest, HistogramRecordRequest, IncrementRequest, LogDrainBuildRequest,
    LogEmitRequest, MetricRegistryBuildRequest, ObserveBootstrap,
    SpanAnnotationRequest, SpanFinishRequest, SpanStartRequest, StdObserveFactory,
    ValidationRequest,
};

fn main() {
    // --- Tracer ---
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "order_handler".to_string(),
            operation: "execute".to_string(),
        })
        .expect("start_span should succeed")
        .span;
    span.record(SpanAnnotationRequest {
        key: "order.id".to_string(),
        value: "ord-42".to_string(),
    })
    .expect("record should succeed");
    span.record(SpanAnnotationRequest {
        key: "order.items".to_string(),
        value: "3".to_string(),
    })
    .expect("record should succeed");
    span.finish(SpanFinishRequest).expect("finish should succeed");

    // --- Metrics ---
    let registry = StdObserveFactory::noop_metric_registry();
    let req_counter = registry
        .counter(CounterLookupRequest {
            name: "handler.requests.total".to_string(),
        })
        .expect("counter lookup should succeed")
        .counter;
    let latency = registry
        .histogram(HistogramLookupRequest {
            name: "handler.latency_ms".to_string(),
        })
        .expect("histogram lookup should succeed")
        .histogram;
    let queue = registry
        .gauge(GaugeLookupRequest {
            name: "handler.queue_depth".to_string(),
        })
        .expect("gauge lookup should succeed")
        .gauge;

    req_counter
        .increment(IncrementRequest { delta: 1 })
        .expect("increment should succeed");
    latency
        .record(HistogramRecordRequest { value: 12.5 })
        .expect("record should succeed");
    queue
        .set(GaugeSetRequest { value: 4.0 })
        .expect("set should succeed");

    // --- Log drain ---
    let drain = StdObserveFactory::noop_log_drain();
    drain
        .emit(LogEmitRequest {
            level: "INFO".to_string(),
            handler_id: "order_handler".to_string(),
            message: "order processed successfully".to_string(),
        })
        .expect("emit should succeed");

    // --- Factory approach ---
    let factory = StdObserveFactory::create_factory();
    assert!(factory.validate(ValidationRequest).is_ok());

    let factory2 = StdObserveFactory::std_factory();
    let _tracer2 = factory2
        .build_handler_tracer(HandlerTracerBuildRequest)
        .expect("build_handler_tracer should succeed");
    let _registry2 = factory2
        .build_metric_registry(MetricRegistryBuildRequest)
        .expect("build_metric_registry should succeed");
    let _drain2 = factory2
        .build_log_drain(LogDrainBuildRequest)
        .expect("build_log_drain should succeed");

    println!("observe example complete — all primitives exercised");
}
