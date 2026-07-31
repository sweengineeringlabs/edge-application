//! `impl Default for EventBusConfig`.

use crate::api::EventBusConfig;

impl Default for EventBusConfig {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capacity_is_1024_happy() {
        assert_eq!(EventBusConfig::default().capacity, 1024);
    }
}
