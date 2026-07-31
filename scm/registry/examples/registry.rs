//! Basic `Registry` usage example.

use std::sync::Arc;

use edge_application_registry::{
    DeregisterRequest, MemoryRegistry, ListIdsRequest, RegisterRequest, Registry, RegistryError,
    RegistryLookupRequest, TryRegisterRequest,
};

fn main() -> Result<(), RegistryError> {
    let registry = MemoryRegistry::<str>::new();
    registry.register(RegisterRequest {
        id: "greeting".to_string(),
        entry: Arc::from("hello"),
    })?;
    registry.register(RegisterRequest {
        id: "farewell".to_string(),
        entry: Arc::from("goodbye"),
    })?;

    if let Some(v) = registry
        .get(RegistryLookupRequest {
            id: "greeting".to_string(),
        })?
        .entry
    {
        println!("greeting = {}", &*v);
    }
    println!("ids = {:?}", registry.list_ids(ListIdsRequest)?.ids);

    // strict registration rejects a duplicate id
    match registry.try_register(TryRegisterRequest {
        id: "greeting".to_string(),
        entry: Arc::from("hi"),
    }) {
        Ok(_) => println!("registered"),
        Err(e) => println!("rejected: {e}"),
    }

    registry.deregister(DeregisterRequest {
        id: "farewell".to_string(),
    })?;
    Ok(())
}
