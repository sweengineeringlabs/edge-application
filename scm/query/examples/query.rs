//! Basic `Query` usage example.

use edge_application_query::{DirectQueryBus, Query, QueryError, QueryExecuteRequest, QueryResultResponse};
use futures::future::BoxFuture;

struct Ping;
impl Query for Ping {
    type Result = String;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Ok(QueryResultResponse { result: "pong".into() }) })
    }
}

fn main() {
    let bus: DirectQueryBus<String> = DirectQueryBus::new();
    let _query: Box<dyn Query<Result = String>> = Box::new(Ping);
    let _ = &bus;
    println!("query sub-crate ready");
}
