//! `Registry` — id-keyed resolution registry of shared entries.

use crate::api::registry::errors::RegistryError;
use crate::api::registry::dto::{
    DeregisterRequest, DeregisterResponse, EmptinessRequest, EmptinessResponse, LenRequest,
    LenResponse, ListIdsRequest, ListIdsResponse, RegisterRequest, RegisterResponse,
    RegistryLookupRequest, RegistryLookupResponse, TryRegisterRequest, TryRegisterResponse,
};

/// An id-keyed registry of shared entries.
///
/// Generalizes the resolution-registry family — handlers, services, and live
/// task controllers: register a shared entry under a string id and resolve it
/// later by id. Concurrent: every method takes `&self`. The stored entry type
/// is the associated [`Value`](Registry::Value) (matching the `Repository` /
/// `ServiceRegistry` convention).
pub trait Registry: Send + Sync {
    /// The (possibly unsized) entry type stored in this registry.
    type Value: ?Sized + Send + Sync;

    /// Register the request's entry under its id, replacing any existing entry.
    fn register(
        &self,
        req: RegisterRequest<Self::Value>,
    ) -> Result<RegisterResponse, RegistryError>;

    /// Register the request's entry under its id, returning
    /// [`RegistryError::DuplicateId`] when an entry is already registered
    /// under that id (the existing entry is left untouched).
    fn try_register(
        &self,
        req: TryRegisterRequest<Self::Value>,
    ) -> Result<TryRegisterResponse, RegistryError>;

    /// Remove the entry registered under the requested id.
    fn deregister(&self, req: DeregisterRequest) -> Result<DeregisterResponse, RegistryError>;

    /// Resolve the entry registered under the requested id.
    fn get(
        &self,
        req: RegistryLookupRequest,
    ) -> Result<RegistryLookupResponse<Self::Value>, RegistryError>;

    /// Return all registered ids.
    fn list_ids(&self, req: ListIdsRequest) -> Result<ListIdsResponse, RegistryError>;

    /// Return the number of registered entries.
    fn len(&self, req: LenRequest) -> Result<LenResponse, RegistryError>;

    /// Return `true` if no entries are registered.
    fn is_empty(&self, _req: EmptinessRequest) -> Result<EmptinessResponse, RegistryError> {
        Ok(EmptinessResponse {
            empty: self.len(LenRequest)?.count == 0,
        })
    }
}
