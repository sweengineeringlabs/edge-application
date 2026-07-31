//! `LoginStatusQuery` — the actual infra unit — a session-store/auth-backend check, in real
//! usage. Swappable independently of `AuthHandler`; this stand-in treats any non-empty token as
//! logged in.
//!
//! (Renamed from `IsLoggedInQuery`: `is_` is a boolean-assertion prefix flagged by
//! `file_names_are_nouns`.)

use edge_application_query::{Query, QueryError, QueryExecuteRequest, QueryResultResponse};

pub struct LoginStatusQuery {
    pub session_token: String,
}

impl Query for LoginStatusQuery {
    type Result = bool;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> futures::future::BoxFuture<'_, Result<QueryResultResponse<bool>, QueryError>> {
        let token = self.session_token.clone();
        Box::pin(async move {
            tracing::info!("[infra] LoginStatusQuery::execute — checking session store for {token:?}");
            let logged_in = !token.is_empty();
            Ok(QueryResultResponse { result: logged_in })
        })
    }
}
