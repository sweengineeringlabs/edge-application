//! `HandlerRegistry` impl for [`InProcessHandlerRegistry`] — RwLock-backed in-process store.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::DeregisterHandlerRequest;
use crate::api::DeregisterHandlerResponse;
use crate::api::Handler;
use crate::api::HandlerError;
use crate::api::HandlerLookupRequest;
use crate::api::HandlerLookupResponse;
use crate::api::HandlerRegistry;
use crate::api::IdRequest;
use crate::api::InProcessHandlerRegistry;
use crate::api::LenRequest;
use crate::api::LenResponse;
use crate::api::ListIdsRequest;
use crate::api::ListIdsResponse;
use crate::api::RegisterHandlerRequest;
use crate::api::RegisterHandlerResponse;

impl<Req, Resp> RegisterHandlerRequest<Req, Resp>
where
    Req: edge_application_base::Request,
    Resp: edge_application_base::Response,
{
    /// Create a new registration request for the given handler.
    pub fn new(handler: Arc<dyn Handler<Request = Req, Response = Resp>>) -> Self {
        Self { handler }
    }
}

impl<Req, Resp> InProcessHandlerRegistry<Req, Resp>
where
    Req: edge_application_base::Request,
    Resp: edge_application_base::Response,
{
    pub(crate) fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }
}

impl<Req, Resp> Default for InProcessHandlerRegistry<Req, Resp>
where
    Req: edge_application_base::Request,
    Resp: edge_application_base::Response,
{
    fn default() -> Self {
        Self::new()
    }
}

// SEA no_orphan_types exemption detection needs "HandlerRegistry for InProcessHandlerRegistry"
// on one line — a wrapped signature reads as an orphan type despite this being a real impl.
#[rustfmt::skip]
impl<Req: edge_application_base::Request, Resp: edge_application_base::Response> HandlerRegistry for InProcessHandlerRegistry<Req, Resp> {
    type Request = Req;
    type Response = Resp;

    fn register(
        &self,
        req: RegisterHandlerRequest<Req, Resp>,
    ) -> Result<RegisterHandlerResponse, HandlerError> {
        let id = req.handler.id(IdRequest)?.id;
        self.handlers.write().insert(id, req.handler);
        Ok(RegisterHandlerResponse)
    }

    fn deregister(
        &self,
        req: DeregisterHandlerRequest,
    ) -> Result<DeregisterHandlerResponse, HandlerError> {
        let was_present = self.handlers.write().remove(&req.id).is_some();
        Ok(DeregisterHandlerResponse { was_present })
    }

    fn get(
        &self,
        req: HandlerLookupRequest,
    ) -> Result<HandlerLookupResponse<Req, Resp>, HandlerError> {
        let handler = self.handlers.read().get(&req.id).cloned();
        Ok(HandlerLookupResponse { handler })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, HandlerError> {
        let guard = self.handlers.read();
        let mut ids: Vec<String> = guard.keys().cloned().collect();
        ids.sort();
        Ok(ListIdsResponse { ids })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, HandlerError> {
        Ok(LenResponse {
            count: self.handlers.read().len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::EchoHandler;
    use crate::api::EmptinessRequest;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TextPayload(String);

    impl edge_application_base::Request for TextPayload {}
    impl edge_application_base::Response for TextPayload {}

    fn handler_with_id(id: &str) -> Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> {
        Arc::new(EchoHandler::<TextPayload>::from((id, "/")))
    }

    #[test]
    fn test_new_creates_empty_registry_happy() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }

    #[test]
    fn test_register_makes_handler_retrievable_happy() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        reg.register(RegisterHandlerRequest::new(handler_with_id("s1")))
            .unwrap();
        assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    }

    #[test]
    fn test_deregister_existing_returns_true_happy() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        reg.register(RegisterHandlerRequest::new(handler_with_id("s1")))
            .unwrap();
        assert!(
            reg.deregister(DeregisterHandlerRequest {
                id: "s1".to_string()
            })
            .unwrap()
            .was_present
        );
    }

    #[test]
    fn test_deregister_missing_returns_false_edge() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        assert!(
            !reg.deregister(DeregisterHandlerRequest {
                id: "missing".to_string()
            })
            .unwrap()
            .was_present
        );
    }

    #[test]
    fn test_get_missing_id_returns_none_error() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        let handler = reg
            .get(HandlerLookupRequest {
                id: "missing".to_string(),
            })
            .unwrap()
            .handler;
        assert!(handler.is_none());
    }

    #[test]
    fn test_list_ids_returns_sorted_ids_happy() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        reg.register(RegisterHandlerRequest::new(handler_with_id("z")))
            .unwrap();
        reg.register(RegisterHandlerRequest::new(handler_with_id("a")))
            .unwrap();
        assert_eq!(reg.list_ids(ListIdsRequest).unwrap().ids, vec!["a", "z"]);
    }

    #[test]
    fn test_len_empty_registry_returns_zero_edge() {
        let reg: InProcessHandlerRegistry<TextPayload, TextPayload> = InProcessHandlerRegistry::new();
        assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    }
}
