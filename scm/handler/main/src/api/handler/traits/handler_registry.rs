//! `HandlerRegistry` trait — stores and retrieves [`Handler`] instances by id.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::dto::{
    DeregisterHandlerRequest, DeregisterHandlerResponse, EmptinessRequest, EmptinessResponse,
    HandlerLookupRequest, HandlerLookupResponse, LenRequest, LenResponse, ListIdsRequest,
    ListIdsResponse, RegisterHandlerRequest, RegisterHandlerResponse,
};

/// A thread-safe registry that stores and retrieves [`Handler`] instances by id.
pub trait HandlerRegistry: Send + Sync {
    /// The request type for handlers stored in this registry.
    type Request: edge_application_base::Request;

    /// The response type for handlers stored in this registry.
    type Response: edge_application_base::Response;

    /// Register a handler, replacing any existing entry with the same id.
    fn register(
        &self,
        req: RegisterHandlerRequest<Self::Request, Self::Response>,
    ) -> Result<RegisterHandlerResponse, HandlerError>;

    /// Remove the handler with the given id. Returns `true` if it existed.
    fn deregister(
        &self,
        req: DeregisterHandlerRequest,
    ) -> Result<DeregisterHandlerResponse, HandlerError>;

    /// Look up a handler by id.
    fn get(
        &self,
        req: HandlerLookupRequest,
    ) -> Result<HandlerLookupResponse<Self::Request, Self::Response>, HandlerError>;

    /// Return all registered handler ids.
    fn list_ids(&self, req: ListIdsRequest) -> Result<ListIdsResponse, HandlerError>;

    /// Return the number of registered handlers.
    fn len(&self, req: LenRequest) -> Result<LenResponse, HandlerError>;

    /// Return `true` if no handlers are registered.
    fn is_empty(&self, _req: EmptinessRequest) -> Result<EmptinessResponse, HandlerError> {
        Ok(EmptinessResponse {
            empty: self.len(LenRequest)?.count == 0,
        })
    }
}
