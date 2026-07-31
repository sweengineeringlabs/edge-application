//! # edge-domain-saga
//!
//! The `Saga` port contract — long-running process manager driven by events and commands.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::MemorySagaStore;
pub use api::NoopSaga;
pub use api::NoopSagaCommand;
pub use api::NoopSagaEvent;
pub use api::SagaCommandDispatchRequest;
pub use api::SagaError;
pub use api::SagaEventDescribeRequest;
pub use api::SagaEventDescribeResponse;
pub use api::SagaGetRequest;
pub use api::SagaGetResponse;
pub use api::SagaHandleRequest;
pub use api::SagaHandleResponse;
pub use api::SagaIsCompleteRequest;
pub use api::SagaIsCompleteResponse;
pub use api::SagaRegisterRequest;
pub use edge_application_command::Command;
pub use edge_application_command::CommandError;
pub use edge_application_event::DomainEvent;
pub use saf::Saga;
pub use saf::SagaCommand;
pub use saf::SagaEvent;
pub use saf::SagaStore;
pub use saf::SAGA_COMMAND_SVC;
pub use saf::SAGA_COMMAND_SVC_FACTORY;
pub use saf::SAGA_EVENT_SVC;
pub use saf::SAGA_EVENT_SVC_FACTORY;
pub use saf::SAGA_STORE_SVC;
pub use saf::SAGA_STORE_SVC_FACTORY;
pub use saf::SAGA_SVC;
pub use saf::SAGA_SVC_FACTORY;
