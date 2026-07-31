//! Integration tests for `EventBusConfig`.

use edge_application_event::EventBusConfig;

/// @covers: EventBusConfig — default capacity is 1024
#[test]
fn test_event_bus_config_default_capacity_is_1024_happy() {
    let cfg = EventBusConfig::default();
    assert_eq!(cfg.capacity, 1024);
}

/// @covers: EventBusConfig — custom capacity is stored
#[test]
fn test_event_bus_config_custom_capacity_stored_error() {
    let cfg = EventBusConfig { capacity: 64 };
    assert_eq!(cfg.capacity, 64);
}

/// @covers: EventBusConfig — clone produces equal config
#[test]
fn test_event_bus_config_clone_is_equal_edge() {
    let cfg = EventBusConfig { capacity: 256 };
    let cloned = cfg.clone();
    assert_eq!(cloned.capacity, cfg.capacity);
}
