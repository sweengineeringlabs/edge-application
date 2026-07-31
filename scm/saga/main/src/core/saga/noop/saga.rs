use crate::api::Saga;
use crate::api::SagaError;
use crate::api::{NoopSaga, NoopSagaCommand, NoopSagaEvent};
use crate::api::{
    SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest, SagaIsCompleteResponse,
};

impl Saga for NoopSaga {
    type SagaId = u64;
    type Event = NoopSagaEvent;
    type Command = NoopSagaCommand;

    fn handle(
        &mut self,
        _req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError> {
        Ok(SagaHandleResponse { commands: vec![] })
    }

    fn is_complete(
        &self,
        _req: SagaIsCompleteRequest,
    ) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.complete,
        })
    }
}
