//! `OrderCreated` — the event type appended to and loaded from the injected
//! `EventStore<OrderCreated>`.

use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError, EventTypeRequest,
    EventTypeResponse,
};

#[derive(Debug, Clone)]
pub struct OrderCreated {
    pub order_id: String,
    pub item: String,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "order.created",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.order_id,
        })
    }
}
