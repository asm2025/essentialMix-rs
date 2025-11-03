use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    unix_time().as_secs()
}

pub fn now_micros() -> u128 {
    unix_time().as_micros()
}

pub fn now_millis() -> u128 {
    unix_time().as_millis()
}

pub fn to_system_time(seconds: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(seconds)
}

fn unix_time() -> Duration {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Invalid time duration")
}
