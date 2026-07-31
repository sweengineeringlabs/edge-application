//! Basic `DomainEvent` usage example.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{DomainEvent, EventAggregateIdRequest, EventTypeRequest, NoopEventBus};

struct OrderCreated {
    order_id: String,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventTypeResponse { event_type: "order.created" })
    }
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<edge_application_event::EventAggregateIdResponse<'_>, edge_application_event::EventError> {
        Ok(edge_application_event::EventAggregateIdResponse { aggregate_id: &self.order_id })
    }
}

fn main() {
    let _bus = NoopEventBus;
    let evt = OrderCreated { order_id: "order-1".into() };
    let event_type = evt.event_type(EventTypeRequest).unwrap().event_type;
    let aggregate_id = evt.aggregate_id(EventAggregateIdRequest).unwrap().aggregate_id;
    println!("event sub-crate ready: {event_type} for {aggregate_id}");
}
