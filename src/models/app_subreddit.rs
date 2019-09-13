#![allow(dead_code, unused_imports, unused_variables)]

/// AppSubreddit
///
/// All data on a single subreddit, independently from any user.

use redis::{Commands, Connection};
use serde::{Serialize, Deserialize};

use crate::jwt;
use crate::helpers::AppResult;

fn get_redis_conn() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let conn = client.get_connection().unwrap();
    conn
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppSubreddit {
    id: String,             // sans "t5_"
    name: String,
    created: String,        // created_utc
    url: String,            // max_length=50; e.g. "/r/de"
    over18: bool,
    lang: String,           // max_length=10; language
    title: String,          // max_length=100
    header_title: String,   // max_length=100
    display_name: String,   // primary name to index
    subreddit_type: String, // "public", "restricted", or "private"
    subscribers: u64,       // subreddit subscribers count
    subscribers_here: u64,  // subreddit subscribers with an account on reddmeet
}

impl AppSubreddit {
    pub fn fetch(token: jwt::JwtTokenToken) -> AppResult<Self> {
        Ok(Self { ..Default::default() })
    }
}
