//! `Handler` impl and construction for [`EchoHandler`].

use std::marker::PhantomData;

use async_trait::async_trait;

use crate::api::EchoHandler;
use crate::api::ExecutionRequest;
use crate::api::Handler;
use crate::api::HandlerError;
use crate::api::IdRequest;
use crate::api::IdResponse;
use crate::api::PatternRequest;
use crate::api::PatternResponse;

impl<T> From<(&str, &str)> for EchoHandler<T> {
    fn from((id, pattern): (&str, &str)) -> Self {
        Self {
            id: id.into(),
            pattern: pattern.into(),
            _marker: PhantomData,
        }
    }
}

// SEA no_orphan_types exemption detection needs "Handler for EchoHandler" on one line —
// a wrapped signature reads as an orphan type despite this being a real impl.
#[rustfmt::skip]
#[async_trait]
impl<T: Clone + edge_application_base::Request + edge_application_base::Response> Handler for EchoHandler<T> {
    type Request = T;
    type Response = T;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.clone(),
        })
    }

    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: self.pattern.clone(),
        })
    }

    async fn execute(&self, req: ExecutionRequest<'_, T>) -> Result<T, HandlerError> {
        Ok(req.req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_stores_id_and_pattern_happy() {
        let h = EchoHandler::<String>::from(("from-echo", "/from"));
        assert_eq!(h.id, "from-echo");
        assert_eq!(h.pattern, "/from");
    }

    #[test]
    fn test_from_empty_id_stores_empty_edge() {
        let h = EchoHandler::<String>::from(("", ""));
        assert_eq!(h.id, "");
    }

    #[test]
    fn test_from_empty_id_fails_handler_id_contract_error() {
        let h = EchoHandler::<String>::from(("", "/"));
        assert_eq!(h.id, "");
    }
}
