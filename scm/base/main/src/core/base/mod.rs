//! `Request`/`Response` — no default implementation for consumer-supplied types; consumers
//! provide their own. [`EmptyRequest`](crate::api::EmptyRequest)/
//! [`EmptyResponse`](crate::api::EmptyResponse) are the one canonical exception, implemented
//! here since this crate owns both the trait and the type.

mod empty_request;
mod empty_response;
