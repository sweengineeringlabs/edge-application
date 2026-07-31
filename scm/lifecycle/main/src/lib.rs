//! # edge-domain-lifecycle
//!
//! The `Lifecycle` port contract — a generic state-machine abstraction with a
//! pluggable [`TransitionPolicy`].
//!
//! Model the lifecycle of any entity (task, connection, saga, actor) without
//! coupling the framework to a specific state vocabulary.  The reference
//! implementation is [`ManagedLifecycle`], driven by [`PermissivePolicy`] or a
//! consumer-supplied policy.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::LifecycleError;
pub use api::LifecycleIsInRequest;
pub use api::LifecycleIsInResponse;
pub use api::LifecycleStateRequest;
pub use api::LifecycleStateResponse;
pub use api::LifecycleTransitionRequest;
pub use api::ManagedLifecycle;
pub use api::PermissivePolicy;
pub use api::TransitionAllowedRequest;
pub use api::TransitionAllowedResponse;
pub use saf::Lifecycle;
pub use saf::TransitionPolicy;
pub use saf::LIFECYCLE_SVC;
pub use saf::LIFECYCLE_SVC_FACTORY;
pub use saf::TRANSITION_POLICY_SVC;
pub use saf::TRANSITION_POLICY_SVC_FACTORY;
