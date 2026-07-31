//! Blanket bridge: every `edge_application_event::DomainEvent` satisfies `SagaEvent`.

use edge_application_event::{DomainEvent, EventAggregateIdRequest, EventTypeRequest};

use crate::api::SagaError;
use crate::api::SagaEvent;
use crate::api::SagaEventDescribeRequest;
use crate::api::SagaEventDescribeResponse;

impl<T: DomainEvent> SagaEvent for T {
    fn describe(
        &self,
        _req: SagaEventDescribeRequest,
    ) -> Result<SagaEventDescribeResponse, SagaError> {
        Ok(SagaEventDescribeResponse {
            event_type: self
                .event_type(EventTypeRequest)
                .map(|r| r.event_type.to_string())
                .unwrap_or_default(),
            aggregate_id: self
                .aggregate_id(EventAggregateIdRequest)
                .map(|r| r.aggregate_id.to_string())
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use edge_application_event::{EventAggregateIdResponse, EventError};

    use super::*;

    struct SagaEventBridgeTestEvt;

    impl DomainEvent for SagaEventBridgeTestEvt {
        fn aggregate_id(
            &self,
            _req: EventAggregateIdRequest,
        ) -> Result<EventAggregateIdResponse<'_>, EventError> {
            Ok(EventAggregateIdResponse { aggregate_id: "agg-1" })
        }
    }

    #[test]
    fn test_describe_domain_event_default_type_returns_event_happy() {
        let e = SagaEventBridgeTestEvt;
        assert_eq!(e.describe(SagaEventDescribeRequest).unwrap().event_type, "event");
    }

    #[test]
    fn test_describe_domain_event_overridden_aggregate_id_returns_agg_1_error() {
        let e = SagaEventBridgeTestEvt;
        assert_eq!(e.describe(SagaEventDescribeRequest).unwrap().aggregate_id, "agg-1");
    }

    #[test]
    fn test_describe_domain_event_default_aggregate_id_returns_empty_edge() {
        #[derive(Clone)]
        struct SagaEventBridgeDefaultTestEvt;
        impl DomainEvent for SagaEventBridgeDefaultTestEvt {}
        assert_eq!(
            SagaEventBridgeDefaultTestEvt
                .describe(SagaEventDescribeRequest)
                .unwrap()
                .aggregate_id,
            ""
        );
    }
}
