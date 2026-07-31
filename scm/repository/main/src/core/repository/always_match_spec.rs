//! [`Spec`] impl for [`AlwaysMatchSpec`].

use std::marker::PhantomData;

use crate::api::RepositoryError;
use crate::api::{AlwaysMatchSpec, Spec, SpecMatchesRequest, SpecMatchesResponse};

impl<T> AlwaysMatchSpec<T> {
    /// Creates a new `AlwaysMatchSpec` that matches every entity of type `T`.
    pub fn new() -> Self {
        Self {
            entity: PhantomData,
        }
    }
}

impl<T> Default for AlwaysMatchSpec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + Sync> Spec for AlwaysMatchSpec<T> {
    type Entity = T;

    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, T>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: true })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_always_returns_true() {
        let result = AlwaysMatchSpec::new().matches(SpecMatchesRequest { entity: &42u32 });
        assert_eq!(
            result.map(|r| r.matches),
            Ok(true),
            "AlwaysMatchSpec should always match"
        );
    }

    #[test]
    fn test_default_matches_always_returns_true_edge() {
        let spec: AlwaysMatchSpec<u32> = AlwaysMatchSpec::default();
        let result = spec.matches(SpecMatchesRequest { entity: &0u32 });
        assert_eq!(
            result.map(|r| r.matches),
            Ok(true),
            "AlwaysMatchSpec::default should always match"
        );
    }
}
