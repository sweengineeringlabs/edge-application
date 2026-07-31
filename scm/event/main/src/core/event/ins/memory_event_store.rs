//! [`EventStore`] impl for [`MemoryEventStore<E>`] — append-only in-memory store.

use std::collections::HashMap;
use std::time::SystemTime;

use futures::future::BoxFuture;
use parking_lot::RwLock;

use crate::api::EventStoreError;
use crate::api::{DomainEvent, EventStore};
use crate::api::{
    EventEnvelope, EventStoreAppendRequest, EventStoreAppendResponse, EventStoreLoadFromRequest,
    EventStoreLoadFromResponse, EventStoreLoadRequest, EventStoreLoadResponse, ExpectedVersion,
    MemoryEventStore,
};

impl<E: DomainEvent + Clone + Send + Sync + 'static> MemoryEventStore<E> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            streams: RwLock::new(HashMap::new()),
        }
    }
}

impl<E: DomainEvent + Clone + Send + Sync + 'static> Default for MemoryEventStore<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> EventStore for MemoryEventStore<E>
where
    E: DomainEvent + Clone + Send + Sync + 'static,
{
    type Event = E;

    fn append(
        &self,
        req: EventStoreAppendRequest<'_, Self::Event>,
    ) -> BoxFuture<'_, Result<EventStoreAppendResponse, EventStoreError>> {
        let aggregate_id = req.aggregate_id.to_owned();
        let events = req.events;
        let expected = req.expected;
        Box::pin(async move {
            let mut streams = self.streams.write();
            let stream = streams.entry(aggregate_id.clone()).or_default();
            let current_version = stream.len() as u64;

            // Optimistic concurrency check
            match expected {
                ExpectedVersion::Any => {}
                ExpectedVersion::NoStream => {
                    if current_version != 0 {
                        return Err(EventStoreError::Conflict {
                            aggregate_id,
                            expected: 0,
                            actual: current_version,
                        });
                    }
                }
                ExpectedVersion::Exact(v) => {
                    if current_version != v {
                        return Err(EventStoreError::Conflict {
                            aggregate_id,
                            expected: v,
                            actual: current_version,
                        });
                    }
                }
            }

            let mut next_seq = current_version + 1;
            for event in events {
                stream.push(EventEnvelope {
                    aggregate_id: aggregate_id.clone(),
                    sequence: next_seq,
                    occurred_at: SystemTime::now(),
                    event,
                });
                next_seq += 1;
            }

            Ok(EventStoreAppendResponse {
                sequence: next_seq - 1,
            })
        })
    }

    fn load(
        &self,
        req: EventStoreLoadRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadResponse<Self::Event>, EventStoreError>> {
        let aggregate_id = req.aggregate_id.to_owned();
        Box::pin(async move {
            let streams = self.streams.read();
            Ok(EventStoreLoadResponse {
                events: streams.get(&aggregate_id).cloned().unwrap_or_default(),
            })
        })
    }

    fn load_from(
        &self,
        req: EventStoreLoadFromRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadFromResponse<Self::Event>, EventStoreError>> {
        let aggregate_id = req.aggregate_id.to_owned();
        let from_sequence = req.from_sequence;
        Box::pin(async move {
            let streams = self.streams.read();
            Ok(EventStoreLoadFromResponse {
                events: streams
                    .get(&aggregate_id)
                    .map(|s| {
                        s.iter()
                            .filter(|e| e.sequence >= from_sequence)
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_default(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{EventAggregateIdRequest, EventAggregateIdResponse, EventError, EventTypeRequest, EventTypeResponse};

    #[derive(Clone)]
    struct MemoryEventStoreEvt(String);
    impl DomainEvent for MemoryEventStoreEvt {
        fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
            Ok(EventTypeResponse { event_type: "evt" })
        }
        fn aggregate_id(
            &self,
            _req: EventAggregateIdRequest,
        ) -> Result<EventAggregateIdResponse<'_>, EventError> {
            Ok(EventAggregateIdResponse {
                aggregate_id: &self.0,
            })
        }
    }

    fn append_req(
        aggregate_id: &str,
        events: Vec<MemoryEventStoreEvt>,
        expected: ExpectedVersion,
    ) -> EventStoreAppendRequest<'_, MemoryEventStoreEvt> {
        EventStoreAppendRequest {
            aggregate_id,
            events,
            expected,
        }
    }

    /// @covers: append
    #[test]
    fn test_append_no_stream_first_write_returns_sequence_happy() {
        let store: MemoryEventStore<MemoryEventStoreEvt> = MemoryEventStore::new();
        let result = futures::executor::block_on(store.append(append_req(
            "agg-1",
            vec![MemoryEventStoreEvt("agg-1".into())],
            ExpectedVersion::NoStream,
        )));
        assert_eq!(result.expect("append").sequence, 1);
    }

    /// @covers: append
    #[test]
    fn test_append_no_stream_when_stream_exists_returns_conflict_error() {
        let store: MemoryEventStore<MemoryEventStoreEvt> = MemoryEventStore::new();
        futures::executor::block_on(store.append(append_req(
            "agg-2",
            vec![MemoryEventStoreEvt("agg-2".into())],
            ExpectedVersion::NoStream,
        )))
        .expect("first append");
        let result = futures::executor::block_on(store.append(append_req(
            "agg-2",
            vec![MemoryEventStoreEvt("agg-2".into())],
            ExpectedVersion::NoStream,
        )));
        assert!(matches!(result, Err(EventStoreError::Conflict { .. })));
    }

    /// @covers: append
    #[test]
    fn test_append_exact_wrong_version_returns_conflict_edge() {
        let store: MemoryEventStore<MemoryEventStoreEvt> = MemoryEventStore::new();
        let result = futures::executor::block_on(store.append(append_req(
            "agg-3",
            vec![MemoryEventStoreEvt("agg-3".into())],
            ExpectedVersion::Exact(99),
        )));
        assert!(matches!(result, Err(EventStoreError::Conflict { .. })));
    }
}
