//! `OrderSaga` — the `Saga` instance shared by `SagaEventHandler` and `SagaCompletionHandler`,
//! wired through `Arc<parking_lot::Mutex<OrderSaga>>`. See the crate root doc comment for why
//! this bypasses `SagaStore`.

use edge_application_saga::{
    Saga, SagaError, SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest,
    SagaIsCompleteResponse,
};

use crate::api::{OrderPaid, ShipOrderCommand};

#[derive(Debug, Default)]
pub struct OrderSaga {
    pub shipped: bool,
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = OrderPaid;
    type Command = ShipOrderCommand;

    fn handle(
        &mut self,
        req: SagaHandleRequest<'_, OrderPaid>,
    ) -> Result<SagaHandleResponse<ShipOrderCommand>, SagaError> {
        self.shipped = true;
        Ok(SagaHandleResponse {
            commands: vec![ShipOrderCommand {
                order_id: req.event.order_id.clone(),
            }],
        })
    }

    fn is_complete(&self, _req: SagaIsCompleteRequest) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.shipped,
        })
    }
}
