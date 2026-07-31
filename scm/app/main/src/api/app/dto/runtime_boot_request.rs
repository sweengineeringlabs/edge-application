//! [`RuntimeBootRequest`] — request to run the `AppRuntime` boot gate.
// @allow: dto_types_must_serialize — holds a `&dyn Bootstrap` reference, not
// wire-format data; a trait object reference cannot derive Serialize/Deserialize.

use crate::api::Bootstrap;

/// Request to execute the boot gate: build an application via `bootstrap`, then run it.
pub struct RuntimeBootRequest<'a> {
    /// The bootstrap that builds the application to run.
    pub bootstrap: &'a dyn Bootstrap,
}
