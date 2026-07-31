#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_observer::{
    HandlerTracer, SpanFinishRequest, SpanStartRequest, StdObserveFactory, HANDLER_TRACER_SVC,
};

#[test]
fn test_noop_handler_tracer_svc_builds_usable_tracer_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    tracer
        .start_span(SpanStartRequest {
            handler_id: "handler_a".to_string(),
            operation: "execute".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "noop handler tracer is ZST");
}

#[test]
fn test_noop_handler_tracer_svc_empty_handler_id_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    tracer
        .start_span(SpanStartRequest {
            handler_id: "".to_string(),
            operation: "".to_string(),
        })
        .unwrap()
        .span
        .finish(SpanFinishRequest)
        .unwrap();
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "noop handler tracer is ZST");
}

#[test]
fn test_noop_handler_tracer_svc_multiple_spans_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    for i in 0..3 {
        tracer
            .start_span(SpanStartRequest {
                handler_id: format!("h{i}"),
                operation: "op".to_string(),
            })
            .unwrap()
            .span
            .finish(SpanFinishRequest)
            .unwrap();
    }
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "noop handler tracer is ZST");
}

#[test]
fn test_handler_tracer_svc_key_namespaced_happy() {
    assert!(HANDLER_TRACER_SVC.starts_with("edge."));
}

#[test]
fn test_handler_tracer_svc_returns_dyn_trait_object() {
    let _tracer: Box<dyn HandlerTracer> = StdObserveFactory::noop_handler_tracer();
}
