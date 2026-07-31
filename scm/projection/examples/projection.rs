//! Basic `Projection` usage example.
#![allow(clippy::expect_used)]

use edge_application_event::DomainEvent;
use edge_application_projection::{
    Projection, ProjectionApplyRequest, ProjectionError, ProjectionReadModelRequest,
};

struct Credited {
    amount: u64,
}
impl DomainEvent for Credited {}

struct Balance {
    total: u64,
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
    ) -> Result<edge_application_projection::ProjectionReadModelResponse<'_, u64>, ProjectionError> {
        Ok(edge_application_projection::ProjectionReadModelResponse { read_model: &self.total })
    }
}

fn main() {
    let mut b = Balance { total: 0 };
    b.apply(ProjectionApplyRequest { event: &Credited { amount: 42 } }).expect("apply should succeed");
    println!("balance: {}", b.read_model(ProjectionReadModelRequest).expect("read_model").read_model);
}
