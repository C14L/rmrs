use std::error::Error;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn unix_timestamp() -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|t| t.as_secs())
}

pub fn unix_timestamp_in_secs_from_now(secs: u64) -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|t| t + Duration::from_secs(secs))
        .map(|t| t.as_secs())
}
