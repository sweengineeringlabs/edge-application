//! Basic `Clock` usage example.

use std::time::{Duration, SystemTime};

use edge_application_clock::{Clock, ClockError, FixedClock, NowRequest, SystemClock};

fn main() -> Result<(), ClockError> {
    let wall = SystemClock;
    println!("now: {:?}", wall.now(NowRequest)?.instant);

    let pinned = FixedClock::new(SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000));
    println!("fixed: {:?}", pinned.now(NowRequest)?.instant);
    Ok(())
}
