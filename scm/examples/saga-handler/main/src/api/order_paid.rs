//! `OrderPaid` — the `SagaEvent` handled by `SagaEventHandler`'s injected `OrderSaga`.

use edge_application_saga::{SagaError, SagaEvent, SagaEventDescribeRequest, SagaEventDescribeResponse};

#[derive(Debug, Clone)]
pub struct OrderPaid {
    pub order_id: String,
}

impl SagaEvent for OrderPaid {
    fn describe(
        &self,
        _req: SagaEventDescribeRequest,
    ) -> Result<SagaEventDescribeResponse, SagaError> {
        Ok(SagaEventDescribeResponse {
            event_type: "order.paid".to_string(),
            aggregate_id: self.order_id.clone(),
        })
    }
}
