//! `ShipOrderCommand` — the `SagaCommand` staged by `OrderSaga::handle` in response to an
//! `OrderPaid` event.

use edge_application_saga::{SagaCommand, SagaCommandDispatchRequest, SagaError};

#[derive(Debug, Clone)]
pub struct ShipOrderCommand {
    pub order_id: String,
}

impl SagaCommand for ShipOrderCommand {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), SagaError>> {
        let order_id = self.order_id.clone();
        Box::pin(async move {
            tracing::info!("      [infra] ShipOrderCommand::dispatch — shipping {order_id:?}");
            Ok(())
        })
    }
}
