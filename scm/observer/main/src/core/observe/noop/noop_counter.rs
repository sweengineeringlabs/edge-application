use crate::api::Counter;
use crate::api::IncrementRequest;
use crate::api::IncrementResponse;
use crate::api::NoopCounter;
use crate::api::ObserveError;

impl Counter for NoopCounter {
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, ObserveError> {
        let _ = req;
        Ok(IncrementResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_delta_discarded_happy() {
        let c = NoopCounter;
        c.increment(IncrementRequest { delta: 1 }).unwrap();
        assert_eq!(std::mem::size_of_val(&c), 0);
    }

    #[test]
    fn test_increment_max_value_no_panic_error() {
        let c = NoopCounter;
        c.increment(IncrementRequest { delta: u64::MAX }).unwrap();
        assert_eq!(std::mem::size_of_val(&c), 0);
    }

    #[test]
    fn test_noop_counter_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopCounter>(), 0);
    }
}
