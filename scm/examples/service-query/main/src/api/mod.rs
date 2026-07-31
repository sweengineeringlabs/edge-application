//! API layer — DTOs and supporting types for `AuthHandler`.

mod dto;
mod login_status_query;

pub use dto::{IsLoggedInRequest, IsLoggedInResponse};
pub use login_status_query::LoginStatusQuery;
