//! `Balance` — the `Projection<Credited, u64>` shared by `CreditPostingHandler` and
//! `BalanceLookupHandler`.

use edge_application_projection::{
    Projection, ProjectionApplyRequest, ProjectionError, ProjectionReadModelRequest,
    ProjectionReadModelResponse,
};

use crate::api::Credited;

#[derive(Debug, Default)]
pub struct Balance {
    pub total: u64,
}

impl Projection for Balance {
    type Event = Credited;
    type ReadModel = u64;

    fn apply(&mut self, req: ProjectionApplyRequest<'_, Credited>) -> Result<(), ProjectionError> {
        self.total += req.event.amount;
        Ok(())
    }

    fn read_model(
        &self,
        _req: ProjectionReadModelRequest,
    ) -> Result<ProjectionReadModelResponse<'_, u64>, ProjectionError> {
        Ok(ProjectionReadModelResponse {
            read_model: &self.total,
        })
    }
}
