// Helper functions
//

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn unix_timestamp() -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|t| t.as_secs())
}
