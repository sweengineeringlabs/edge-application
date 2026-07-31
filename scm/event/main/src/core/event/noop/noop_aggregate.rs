//! `impl Aggregate for NoopAggregate`.

use crate::api::Aggregate;
use crate::api::{NoopAggregate, NoopDomainEvent};

impl Aggregate for NoopAggregate {
    type Event = NoopDomainEvent;
}
