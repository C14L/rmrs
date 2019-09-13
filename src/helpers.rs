// #[macro_use]
// extern crate diesel;
// extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn db_establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No database URL set!");
    print!(">>> Connecting DB: {}\n", database_url);
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

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
