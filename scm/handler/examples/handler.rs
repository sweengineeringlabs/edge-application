//! Basic `Handler` usage example.

use async_trait::async_trait;
use edge_application_handler::{
    EchoHandler, ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse, PatternRequest,
    PatternResponse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct Greet;

#[async_trait]
impl Handler for Greet {
    type Request = TextPayload;
    type Response = TextPayload;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "greet".to_string(),
        })
    }
    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: "/greet".to_string(),
        })
    }
    async fn execute(
        &self,
        req: ExecutionRequest<'_, TextPayload>,
    ) -> Result<TextPayload, HandlerError> {
        let greeting = format!("Hello, {}!", req.req.0);
        Ok(TextPayload(greeting))
    }
}

fn main() {
    let _echo: EchoHandler<String> = EchoHandler::from(("echo", "/echo"));
    let _greet = Greet;
    println!("handler sub-crate ready");
}
