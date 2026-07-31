//! `DomainExtensionHealthRequest` — request envelope for [`DomainExtension::health`](crate::api::DomainExtension::health).

/// Zero-sized request for a [`DomainExtension`](crate::api::DomainExtension) health check.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DomainExtensionHealthRequest;
