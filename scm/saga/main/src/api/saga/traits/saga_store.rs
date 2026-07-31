use crate::api::saga::errors::SagaError;
use crate::api::saga::traits::Saga;
use crate::api::saga::dto::{SagaGetRequest, SagaGetResponse, SagaRegisterRequest};

/// Stores live [`Saga`] instances keyed by their `SagaId`.
pub trait SagaStore: Send + Sync {
    /// The concrete saga type stored in this store.
    type SagaInstance: Saga;

    /// Register a saga instance under `id`.
    ///
    /// Returns [`SagaError::AlreadyRegistered`] if a saga is already stored
    /// under `id`; the existing instance is left untouched.
    fn register(
        &mut self,
        req: SagaRegisterRequest<<Self::SagaInstance as Saga>::SagaId, Self::SagaInstance>,
    ) -> Result<(), SagaError>;

    /// Borrow the saga registered under `id`.
    ///
    /// Returns [`SagaError::NotFound`] when no saga is registered under `id`.
    fn get<'a>(
        &'a self,
        req: SagaGetRequest<'a, <Self::SagaInstance as Saga>::SagaId>,
    ) -> Result<SagaGetResponse<'a, Self::SagaInstance>, SagaError>;
}
