use std::collections::HashMap;
use std::fmt::Display;

use crate::api::MemorySagaStore;
use crate::api::SagaError;
use crate::api::{Saga, SagaStore};
use crate::api::{SagaGetRequest, SagaGetResponse, SagaRegisterRequest};

impl<S: Saga> MemorySagaStore<S> {
    /// Construct an empty store.
    pub fn new() -> Self {
        Self {
            sagas: HashMap::new(),
        }
    }
}

impl<S: Saga> Default for MemorySagaStore<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> SagaStore for MemorySagaStore<S>
where
    S: Saga,
    S::SagaId: Display,
{
    type SagaInstance = S;

    fn register(&mut self, req: SagaRegisterRequest<S::SagaId, S>) -> Result<(), SagaError> {
        if self.sagas.contains_key(&req.id) {
            return Err(SagaError::AlreadyRegistered(req.id.to_string()));
        }
        self.sagas.insert(req.id, req.saga);
        Ok(())
    }

    fn get<'a>(
        &'a self,
        req: SagaGetRequest<'a, S::SagaId>,
    ) -> Result<SagaGetResponse<'a, S>, SagaError> {
        self.sagas
            .get(req.id)
            .map(|saga| SagaGetResponse { saga })
            .ok_or_else(|| SagaError::NotFound(req.id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest, SagaIsCompleteResponse,
    };
    use edge_application_command::{Command, CommandError, ExecutionRequest};
    use edge_application_event::DomainEvent;
    use futures::future::BoxFuture;

    #[derive(Debug, Default)]
    struct MemorySagaStoreTestSaga {
        done: bool,
    }

    #[derive(Clone)]
    struct MemorySagaStoreTestSagaSignal;

    impl DomainEvent for MemorySagaStoreTestSagaSignal {
        fn aggregate_id(
            &self,
            _req: edge_application_event::EventAggregateIdRequest,
        ) -> Result<edge_application_event::EventAggregateIdResponse<'_>, edge_application_event::EventError>
        {
            Ok(edge_application_event::EventAggregateIdResponse {
                aggregate_id: "saga-test",
            })
        }
    }

    impl Command for MemorySagaStoreTestSagaSignal {
        fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async move { Ok(()) })
        }
    }

    impl Saga for MemorySagaStoreTestSaga {
        type SagaId = String;
        type Event = MemorySagaStoreTestSagaSignal;
        type Command = MemorySagaStoreTestSagaSignal;

        fn handle(
            &mut self,
            _req: SagaHandleRequest<'_, Self::Event>,
        ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
            self.done = true;
            Ok(SagaHandleResponse { commands: vec![] })
        }

        fn is_complete(
            &self,
            _req: SagaIsCompleteRequest,
        ) -> Result<SagaIsCompleteResponse, SagaError> {
            Ok(SagaIsCompleteResponse {
                complete: self.done,
            })
        }
    }

    type TestStore = MemorySagaStore<MemorySagaStoreTestSaga>;

    #[test]
    fn test_register_then_get_returns_ok() {
        let mut store = TestStore::new();
        store
            .register(SagaRegisterRequest {
                id: "s1".to_string(),
                saga: MemorySagaStoreTestSaga::default(),
            })
            .ok();
        let id = "s1".to_string();
        let saga = store.get(SagaGetRequest { id: &id }).unwrap().saga;
        assert!(
            !saga.is_complete(SagaIsCompleteRequest).unwrap().complete,
            "newly registered saga should not be complete"
        );
    }

    #[test]
    fn test_register_duplicate_returns_already_registered() {
        let mut store = TestStore::new();
        store
            .register(SagaRegisterRequest {
                id: "s1".to_string(),
                saga: MemorySagaStoreTestSaga::default(),
            })
            .ok();
        let err = store
            .register(SagaRegisterRequest {
                id: "s1".to_string(),
                saga: MemorySagaStoreTestSaga::default(),
            })
            .unwrap_err();
        assert_eq!(err, SagaError::AlreadyRegistered("s1".to_string()));
    }

    #[test]
    fn test_get_unknown_id_returns_not_found() {
        let store = TestStore::new();
        let err = store
            .get(SagaGetRequest {
                id: &"ghost".to_string(),
            })
            .unwrap_err();
        assert_eq!(err, SagaError::NotFound("ghost".to_string()));
    }
}
