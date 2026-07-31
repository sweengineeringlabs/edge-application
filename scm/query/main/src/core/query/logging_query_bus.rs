//! `QueryBus` impl for `LoggingQueryBus` — delegates to inner bus, logs via `tracing`.

use futures::future::BoxFuture;

use crate::api::QueryBus;
use crate::api::LoggingQueryBus;
use crate::api::QueryError;
use crate::api::{QueryDispatchRequest, QueryNameRequest, QueryResultResponse};

impl<R: Send + 'static> QueryBus for LoggingQueryBus<R> {
    type Result = R;

    fn dispatch(
        &self,
        req: QueryDispatchRequest<R>,
    ) -> BoxFuture<'_, Result<QueryResultResponse<R>, QueryError>> {
        let name = req
            .query
            .name(QueryNameRequest)
            .map(|r| r.name.to_owned())
            .unwrap_or_else(|_| "unknown".to_owned());
        let inner = self.inner.clone();
        let query = req.query;
        Box::pin(async move {
            let result = inner.dispatch(QueryDispatchRequest { query }).await;
            match &result {
                Ok(_) => tracing::debug!(query = %name, "query dispatched ok"),
                Err(e) => tracing::debug!(query = %name, error = %e, "query dispatch error"),
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;

    use super::*;
    use crate::api::{DirectQueryBus, NoopQueryBus, Query, QueryExecuteRequest};

    struct LoggingQueryBusOk(String);
    impl Query for LoggingQueryBusOk {
        type Result = String;

        fn name(&self, _req: QueryNameRequest) -> Result<crate::api::QueryNameResponse<'_>, QueryError> {
            Ok(crate::api::QueryNameResponse { name: "ok-query" })
        }
        fn execute(
            &self,
            _req: QueryExecuteRequest,
        ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(QueryResultResponse { result: v }) })
        }
    }

    struct LoggingQueryBusErr;
    impl Query for LoggingQueryBusErr {
        type Result = String;

        fn name(&self, _req: QueryNameRequest) -> Result<crate::api::QueryNameResponse<'_>, QueryError> {
            Ok(crate::api::QueryNameResponse { name: "err-query" })
        }
        fn execute(
            &self,
            _req: QueryExecuteRequest,
        ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
            Box::pin(async { Err(QueryError::NotFound("none".into())) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_query_returns_value() {
        let bus = LoggingQueryBus::<String> { inner: Arc::new(DirectQueryBus::new()) };
        let result = bus
            .dispatch(QueryDispatchRequest { query: Box::new(LoggingQueryBusOk("pong".into())) })
            .await;
        assert_eq!(result.unwrap().result, "pong");
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_error_query_propagates_err() {
        let bus = LoggingQueryBus::<String> { inner: Arc::new(NoopQueryBus::new()) };
        assert!(bus
            .dispatch(QueryDispatchRequest { query: Box::new(LoggingQueryBusErr) })
            .await
            .is_err());
    }
}
