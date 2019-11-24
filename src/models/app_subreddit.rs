// #![allow(dead_code, unused_imports, unused_variables)]

extern crate diesel;

// use chrono::naive::NaiveDate;
use diesel::prelude::*;
// use redis::{Commands, Connection};
use serde::{Serialize, Deserialize};

use crate::jwt;
use crate::models::*;
use crate::helpers::AppResult;
use crate::helpers::db_establish_connection;

// fn get_redis_conn() -> Connection {
//     let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
//     let conn = client.get_connection().unwrap();
//     conn
// }

#[derive(Debug, Default, Deserialize, Serialize, Queryable)]
pub struct AppSubreddit {
    id: String,             // sans "t5_"
    name: String,
    // created: NaiveDate,     // created_utc
    url: String,            // max_length=50; e.g. "/r/de"
    over18: bool,
    lang: String,           // max_length=10; language
    title: String,          // max_length=100
    header_title: String,   // max_length=100
    display_name: String,   // primary name to index
    subreddit_type: String, // "public", "restricted", or "private"
    subscribers: i32,       // subreddit subscribers count
    subscribers_here: i32,  // subreddit subscribers with an account on reddmeet
}

impl AppSubreddit {
    pub fn fetch_for_user(_token: jwt::JwtTokenToken) -> AppResult<Self> {
        // TODO: fetch subreddit data from Reddit.

        Ok(Self { ..Default::default() })
    }

    pub fn load() -> AppResult<Self> {
        use self::schema::sr::dsl::*;

        let conn = db_establish_connection();

        let res = sr
            // .filter(published.eq(true))
            .limit(5)
            .load::<Self>(&conn)
            .expect("Error loading data");

        println!(">>> DB result: {:?}", &res);

        Ok(Self { ..Default::default() })
    }
}
