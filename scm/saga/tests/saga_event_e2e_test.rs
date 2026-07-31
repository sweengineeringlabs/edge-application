//! SAF facade tests — `SagaEvent` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_saga::{NoopSagaEvent, SagaError, SagaEvent, SagaEventDescribeRequest, SagaEventDescribeResponse};

struct ConfiguredEvt {
    event_type: &'static str,
    aggregate_id: &'static str,
}

impl SagaEvent for ConfiguredEvt {
    fn describe(&self, _req: SagaEventDescribeRequest) -> Result<SagaEventDescribeResponse, SagaError> {
        Ok(SagaEventDescribeResponse {
            event_type: self.event_type.to_string(),
            aggregate_id: self.aggregate_id.to_string(),
        })
    }
}

struct FailingEvt;

impl SagaEvent for FailingEvt {
    fn describe(&self, _req: SagaEventDescribeRequest) -> Result<SagaEventDescribeResponse, SagaError> {
        Err(SagaError::CommandDispatchFailed("description unavailable".into()))
    }
}

/// @covers: SagaEvent::describe — configured values returned
#[test]
fn test_describe_configured_event_returns_response_happy() {
    let e = ConfiguredEvt {
        event_type: "order.created",
        aggregate_id: "order-1",
    };
    let response = e.describe(SagaEventDescribeRequest).expect("describe should succeed");
    assert_eq!(response.event_type, "order.created");
    assert_eq!(response.aggregate_id, "order-1");
}

/// @covers: SagaEvent::describe — implementor may fail
#[test]
fn test_describe_failing_impl_returns_err_error() {
    let err = FailingEvt.describe(SagaEventDescribeRequest).unwrap_err();
    assert_eq!(
        err,
        SagaError::CommandDispatchFailed("description unavailable".into())
    );
}

/// @covers: SagaEvent::describe — via dyn dispatch
#[test]
fn test_describe_via_dyn_dispatch_returns_response_edge() {
    let e: &dyn SagaEvent = &ConfiguredEvt {
        event_type: "x",
        aggregate_id: "",
    };
    let response = e.describe(SagaEventDescribeRequest).expect("describe should succeed");
    assert_eq!(response.event_type, "x");
    assert_eq!(response.aggregate_id, "");
}

/// @covers: SagaEvent::noop — returns NoopSagaEvent
#[test]
fn test_noop_returns_noop_saga_event_edge() {
    let e: NoopSagaEvent = ConfiguredEvt::noop();
    assert_eq!(e.describe(SagaEventDescribeRequest).unwrap().event_type, "event");
}
