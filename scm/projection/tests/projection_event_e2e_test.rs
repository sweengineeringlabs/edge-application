//! SAF facade tests — `ProjectionEvent` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_projection::{
    ProjectionError, ProjectionEvent, ProjectionEventDescribeRequest, ProjectionEventDescribeResponse,
};

struct ConfiguredEvt {
    event_type: &'static str,
    aggregate_id: &'static str,
}

impl ProjectionEvent for ConfiguredEvt {
    fn describe(
        &self,
        _req: ProjectionEventDescribeRequest,
    ) -> Result<ProjectionEventDescribeResponse, ProjectionError> {
        Ok(ProjectionEventDescribeResponse {
            event_type: self.event_type.to_string(),
            aggregate_id: self.aggregate_id.to_string(),
        })
    }
}

struct FailingEvt;

impl ProjectionEvent for FailingEvt {
    fn describe(
        &self,
        _req: ProjectionEventDescribeRequest,
    ) -> Result<ProjectionEventDescribeResponse, ProjectionError> {
        Err(ProjectionError::Internal("description unavailable".into()))
    }
}

/// @covers: ProjectionEvent::describe — configured values returned
#[test]
fn test_describe_configured_event_returns_response_happy() {
    let e = ConfiguredEvt {
        event_type: "order.created",
        aggregate_id: "order-1",
    };
    let response = e.describe(ProjectionEventDescribeRequest).expect("describe should succeed");
    assert_eq!(response.event_type, "order.created");
    assert_eq!(response.aggregate_id, "order-1");
}

/// @covers: ProjectionEvent::describe — implementor may fail
#[test]
fn test_describe_failing_impl_returns_err_error() {
    let err = FailingEvt.describe(ProjectionEventDescribeRequest).unwrap_err();
    assert_eq!(err, ProjectionError::Internal("description unavailable".into()));
}

/// @covers: ProjectionEvent::describe — via dyn dispatch
#[test]
fn test_describe_via_dyn_dispatch_returns_response_edge() {
    let e: &dyn ProjectionEvent = &ConfiguredEvt {
        event_type: "x",
        aggregate_id: "",
    };
    let response = e.describe(ProjectionEventDescribeRequest).expect("describe should succeed");
    assert_eq!(response.event_type, "x");
    assert_eq!(response.aggregate_id, "");
}
