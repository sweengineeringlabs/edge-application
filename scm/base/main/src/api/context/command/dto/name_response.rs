//! [`NameResponse`] — output of [`Command::name`](super::super::traits::Command::name).

/// Resolved stable name of a [`Command`](super::super::traits::Command).
#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NameResponse {
    /// The command's stable name.
    pub name: String,
}
