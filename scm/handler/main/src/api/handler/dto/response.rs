//! [`Response`] — re-export of the shared response contract every
//! [`Handler::Response`](crate::api::Handler::Response)/
//! [`Service::Response`](crate::api::Service::Response) associated type is bound against.
//!
//! `Handler`/`Service` are generic over their response payload — there is no single concrete
//! response struct to file here per-implementor. What every implementor's `Self::Response`
//! actually resolves to is this shared contract, so that's what this file names.

pub use edge_application_base::Response;
