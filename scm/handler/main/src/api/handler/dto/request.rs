//! [`Request`] — re-export of the shared request contract every
//! [`Handler::Request`](crate::api::Handler::Request)/
//! [`Service::Request`](crate::api::Service::Request) associated type is bound against.
//!
//! `Handler`/`Service` are generic over their request payload — there is no single concrete
//! request struct to file here per-implementor. What every implementor's `Self::Request`
//! actually resolves to is this shared contract, so that's what this file names.

pub use edge_application_base::Request;
