//! `Registry` impl for `MemoryRegistry`.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::api::DeregisterRequest;
use crate::api::DeregisterResponse;
use crate::api::MemoryRegistry;
use crate::api::LenRequest;
use crate::api::LenResponse;
use crate::api::ListIdsRequest;
use crate::api::ListIdsResponse;
use crate::api::RegisterRequest;
use crate::api::RegisterResponse;
use crate::api::Registry;
use crate::api::RegistryError;
use crate::api::RegistryLookupRequest;
use crate::api::RegistryLookupResponse;
use crate::api::TryRegisterRequest;
use crate::api::TryRegisterResponse;

impl<V: ?Sized + Send + Sync> MemoryRegistry<V> {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
}

impl<V: ?Sized + Send + Sync> Default for MemoryRegistry<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: ?Sized + Send + Sync> Registry for MemoryRegistry<V> {
    type Value = V;

    fn register(&self, req: RegisterRequest<V>) -> Result<RegisterResponse, RegistryError> {
        // A poisoned lock means a prior holder panicked; the map is still
        // structurally sound, so recover the guard rather than propagate.
        let mut map = self.entries.write().unwrap_or_else(|e| e.into_inner());
        map.insert(req.id, req.entry);
        Ok(RegisterResponse)
    }

    fn try_register(
        &self,
        req: TryRegisterRequest<V>,
    ) -> Result<TryRegisterResponse, RegistryError> {
        let mut map = self.entries.write().unwrap_or_else(|e| e.into_inner());
        if map.contains_key(&req.id) {
            return Err(RegistryError::DuplicateId(req.id));
        }
        map.insert(req.id, req.entry);
        Ok(TryRegisterResponse)
    }

    fn deregister(&self, req: DeregisterRequest) -> Result<DeregisterResponse, RegistryError> {
        let mut map = self.entries.write().unwrap_or_else(|e| e.into_inner());
        let was_present = map.remove(&req.id).is_some();
        Ok(DeregisterResponse { was_present })
    }

    fn get(
        &self,
        req: RegistryLookupRequest,
    ) -> Result<RegistryLookupResponse<V>, RegistryError> {
        let map = self.entries.read().unwrap_or_else(|e| e.into_inner());
        let entry = map.get(&req.id).map(Arc::clone);
        Ok(RegistryLookupResponse { entry })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, RegistryError> {
        let map = self.entries.read().unwrap_or_else(|e| e.into_inner());
        let ids = map.keys().cloned().collect();
        Ok(ListIdsResponse { ids })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, RegistryError> {
        let map = self.entries.read().unwrap_or_else(|e| e.into_inner());
        Ok(LenResponse { count: map.len() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::EmptinessRequest;

    fn register(reg: &MemoryRegistry<str>, id: &str, entry: &str) {
        reg.register(RegisterRequest {
            id: id.to_string(),
            entry: Arc::from(entry),
        })
        .unwrap();
    }

    #[test]
    fn test_new_creates_empty_registry_happy() {
        let reg: MemoryRegistry<str> = MemoryRegistry::new();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }

    #[test]
    fn test_default_creates_empty_registry_edge() {
        let reg: MemoryRegistry<str> = MemoryRegistry::default();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }

    #[test]
    fn test_register_then_get_returns_entry_happy() {
        let reg: MemoryRegistry<str> = MemoryRegistry::new();
        register(&reg, "a", "alpha");
        let entry = reg
            .get(RegistryLookupRequest {
                id: "a".to_string(),
            })
            .unwrap()
            .entry;
        assert_eq!(entry.as_deref(), Some("alpha"));
    }

    #[test]
    fn test_register_replaces_existing_entry_edge() {
        let reg: MemoryRegistry<str> = MemoryRegistry::new();
        register(&reg, "a", "alpha");
        register(&reg, "a", "beta");
        let entry = reg
            .get(RegistryLookupRequest {
                id: "a".to_string(),
            })
            .unwrap()
            .entry;
        assert_eq!(entry.as_deref(), Some("beta"));
        assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    }

    #[test]
    fn test_try_register_duplicate_returns_err_error() {
        let reg: MemoryRegistry<str> = MemoryRegistry::new();
        register(&reg, "a", "alpha");
        let result = reg.try_register(TryRegisterRequest {
            id: "a".to_string(),
            entry: Arc::from("beta"),
        });
        assert_eq!(result, Err(RegistryError::DuplicateId("a".to_string())));
        // original entry untouched
        let entry = reg
            .get(RegistryLookupRequest {
                id: "a".to_string(),
            })
            .unwrap()
            .entry;
        assert_eq!(entry.as_deref(), Some("alpha"));
    }

    #[test]
    fn test_deregister_removes_entry_and_reports_presence_happy() {
        let reg: MemoryRegistry<str> = MemoryRegistry::new();
        register(&reg, "a", "alpha");
        assert!(
            reg.deregister(DeregisterRequest {
                id: "a".to_string()
            })
            .unwrap()
            .was_present
        );
        assert!(
            !reg.deregister(DeregisterRequest {
                id: "a".to_string()
            })
            .unwrap()
            .was_present
        );
        let entry = reg
            .get(RegistryLookupRequest {
                id: "a".to_string(),
            })
            .unwrap()
            .entry;
        assert!(entry.is_none());
    }

    #[test]
    fn test_list_ids_and_len_reflect_contents_happy() {
        let reg: MemoryRegistry<str> = MemoryRegistry::new();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
        register(&reg, "a", "alpha");
        register(&reg, "b", "beta");
        let mut ids = reg.list_ids(ListIdsRequest).unwrap().ids;
        ids.sort();
        assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(reg.len(LenRequest).unwrap().count, 2);
    }
}
