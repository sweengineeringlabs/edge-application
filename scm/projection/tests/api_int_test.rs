//! Layer-level coverage for `api/projection/dto/*.rs` request/response types.

use edge_application_projection::{
    ProjectionApplyRequest, ProjectionReadModelRequest, ProjectionReadModelResponse, TryDrainResponse,
};

/// @covers: ProjectionApplyRequest
#[test]
fn test_projection_apply_request_holds_event_happy() {
    let event = 42u32;
    let r = ProjectionApplyRequest { event: &event };
    assert_eq!(*r.event, 42);
}

/// @covers: ProjectionReadModelRequest
#[test]
fn test_projection_read_model_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ProjectionReadModelRequest>(), 0);
    let _ = ProjectionReadModelRequest;
}

/// @covers: ProjectionReadModelResponse
#[test]
fn test_projection_read_model_response_holds_read_model_happy() {
    let model = 7u64;
    let r = ProjectionReadModelResponse { read_model: &model };
    assert_eq!(*r.read_model, 7);
}

/// @covers: TryDrainResponse
#[test]
fn test_try_drain_response_holds_count_error() {
    let r = TryDrainResponse { count: 0 };
    assert_eq!(r.count, 0);
}
