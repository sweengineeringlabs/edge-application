use crate::api::projection::errors::ProjectionError;
use crate::api::projection::traits::ProjectionEvent;
use crate::api::projection::dto::{
    ProjectionApplyRequest, ProjectionReadModelRequest, ProjectionReadModelResponse, TryDrainRequest,
    TryDrainResponse,
};

/// Consumes domain events and maintains a read model.
///
/// The read side of CQRS: apply events in stream order to build a
/// denormalised view optimised for queries.
pub trait Projection: Send + Sync {
    /// The event type this projection handles.
    type Event: ProjectionEvent;

    /// The read model this projection maintains.
    type ReadModel;

    /// Apply one event to update the read model in place.
    fn apply(&mut self, req: ProjectionApplyRequest<'_, Self::Event>) -> Result<(), ProjectionError>;

    /// Return the current read model state.
    fn read_model(
        &self,
        req: ProjectionReadModelRequest,
    ) -> Result<ProjectionReadModelResponse<'_, Self::ReadModel>, ProjectionError>;

    /// Fold a slice of events by calling [`apply`](Self::apply) once per event.
    ///
    /// Returns [`ProjectionError::EmptyStream`] when `events` is empty.
    fn try_drain(
        &mut self,
        req: TryDrainRequest<'_, Self::Event>,
    ) -> Result<TryDrainResponse, ProjectionError>
    where
        Self: Sized,
    {
        if req.events.is_empty() {
            return Err(ProjectionError::EmptyStream);
        }
        for event in req.events {
            self.apply(ProjectionApplyRequest { event })?;
        }
        Ok(TryDrainResponse { count: req.events.len() })
    }
}
