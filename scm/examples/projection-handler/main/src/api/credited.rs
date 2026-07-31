//! `Credited` — the event type folded into the injected `Projection<Credited, u64>`.

use edge_application_projection::{
    ProjectionError, ProjectionEvent, ProjectionEventDescribeRequest, ProjectionEventDescribeResponse,
};

#[derive(Debug, Clone, Copy)]
pub struct Credited {
    pub amount: u64,
}

impl ProjectionEvent for Credited {
    fn describe(
        &self,
        _req: ProjectionEventDescribeRequest,
    ) -> Result<ProjectionEventDescribeResponse, ProjectionError> {
        Ok(ProjectionEventDescribeResponse {
            event_type: "credited".to_string(),
            aggregate_id: "balance".to_string(),
        })
    }
}
