//! `QueryBus` impl for [`DirectQueryBus`] — inline dispatch, no queuing.

use std::marker::PhantomData;

use futures::future::BoxFuture;

use crate::api::QueryBus;
use crate::api::DirectQueryBus;
use crate::api::QueryError;
use crate::api::{QueryDispatchRequest, QueryExecuteRequest, QueryResultResponse};

impl<R> DirectQueryBus<R> {
    /// Construct a new `DirectQueryBus` for result type `R`.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> Default for DirectQueryBus<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Send + 'static> QueryBus for DirectQueryBus<R> {
    type Result = R;

    fn dispatch(
        &self,
        req: QueryDispatchRequest<R>,
    ) -> BoxFuture<'_, Result<QueryResultResponse<R>, QueryError>> {
        Box::pin(async move { req.query.execute(QueryExecuteRequest).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Query;

    struct DirectQueryBusOk(String);
    impl Query for DirectQueryBusOk {
        type Result = String;

        fn execute(
            &self,
            _req: QueryExecuteRequest,
        ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(QueryResultResponse { result: v }) })
        }
    }

    #[test]
    fn test_dispatch_ok_query_returns_value() {
        let bus = DirectQueryBus::<String>::new();
        let result = futures::executor::block_on(
            bus.dispatch(QueryDispatchRequest { query: Box::new(DirectQueryBusOk("pong".into())) })
        );
        assert_eq!(result.unwrap().result, "pong");
    }
}
