// Data models for this app
//
#![allow(dead_code)]

// extern crate redis;

use serde::{Serialize, Deserialize};
use redis::{Commands, Connection};

use crate::jwt;
use crate::models;

fn get_redis_conn() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let conn = client.get_connection().unwrap();
    conn
}

#[derive(Default, Serialize, Deserialize)]
pub struct AppUser {
    pub created: f64,
    pub name: String,
}

impl AppUser {
    pub fn from_reddit(reddit_user: &models::reddit_user::RedditUserMe) -> Option<Self> {
        Some(Self {
            created: reddit_user.created.to_owned(),
            name: reddit_user.name.to_owned(),
        })
    }

    pub fn from_jwt(token: &jwt::JwtTokenToken) -> Option<Self> {
        Some(Self {
            created: token.created.to_owned(),
            name: token.username.to_owned(),
        })
    }

    pub fn load(name: &String) -> Option<Self> {
        let mut conn = get_redis_conn(); // TODO: use connection pool
        let key = format!("rmrs-user-{}", &name.to_ascii_lowercase());
        let res: String = conn.get(&key).unwrap();
        let val: Self = serde_json::from_str(&res).unwrap();
        Some(val)
    }

    pub fn save(&self) {
        let mut conn = get_redis_conn(); // TODO: use connection pool
        let val = serde_json::to_string(&self).unwrap();
        let key = format!("rmrs-user-{}", &self.name.to_ascii_lowercase());
        let _res: String = conn.set(&key, &val).unwrap();
    }
}
