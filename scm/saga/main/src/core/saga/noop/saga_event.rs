use edge_application_event::DomainEvent;

use crate::api::NoopSagaEvent;

impl DomainEvent for NoopSagaEvent {}
