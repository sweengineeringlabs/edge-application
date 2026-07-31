//! [`ListIdsResponse`] — response for [`HandlerRegistry::list_ids`](crate::api::handler::traits::HandlerRegistry::list_ids).

/// All registered handler ids.
pub struct ListIdsResponse {
    /// The ids of all registered handlers.
    pub ids: Vec<String>,
}
