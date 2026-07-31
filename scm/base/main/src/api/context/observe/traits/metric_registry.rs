//! `MetricRegistry` — metric instrument factory.

use crate::api::context::observe::errors::ObserveError;
use crate::api::context::observe::dto::{
    CounterLookupRequest, CounterLookupResponse, GaugeLookupRequest, GaugeLookupResponse,
    HistogramLookupRequest, HistogramLookupResponse,
};

/// Creates named metric instruments.
pub trait MetricRegistry: Send + Sync {
    /// Return a counter for `name`.
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, ObserveError>;

    /// Return a histogram for `name`.
    fn histogram(
        &self,
        req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, ObserveError>;

    /// Return a gauge for `name`.
    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, ObserveError>;
}
