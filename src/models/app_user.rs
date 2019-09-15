#![allow(dead_code)]

extern crate diesel;

use redis::{Commands, Connection};
use serde::{Serialize, Deserialize};

use crate::jwt::JwtTokenToken;
use crate::models::app_subreddit::AppSubreddit;
use crate::models::reddit_user::RedditUser;
use crate::helpers::AppResult;

fn get_redis_conn() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let conn = client.get_connection().unwrap();
    conn
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppUser {
    pub name: String,

    #[serde(default)]
    pub created: f64,

    #[serde(default)]
    pub subs: Vec<AppUserSub>,

    #[serde(default)]
    pub pics: Vec<AppUserPic>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppUserPic {
    url: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppUserSub {
    pub username: String,
    pub subreddit: AppSubreddit,

    #[serde(default)]
    pub is_contributor: bool,

    #[serde(default)]
    pub is_moderator: bool,

    #[serde(default)]
    pub is_subscriber: bool,

    #[serde(default)]
    pub is_banned: bool,

    #[serde(default)]
    pub is_muted: bool,

    #[serde(default)]
    pub is_favorite: bool,
}

impl AppUser {
    /// Instantiate an AppUser from a RedditUser object.
    pub fn from_reddit(reddit_user: &RedditUser) -> AppResult<Self> {
        Ok(Self {
            created: reddit_user.created.to_owned(),
            name: reddit_user.name.to_owned(),
            ..Default::default()
        })
    }

    /// Instantiate an AppUser from a JWT object.
    pub fn from_jwt(token: &JwtTokenToken) -> AppResult<Self> {
        Ok(Self {
            created: token.created.to_owned(),
            name: token.username.to_owned(),
            ..Default::default()
        })
    }

    /// Load AppUser data from storage by user name.
    pub fn load(name: &String) -> AppResult<Self> {
        let mut conn = get_redis_conn(); // TODO: use connection pool
        let key = format!("rmrs-user-{}", &name.to_ascii_lowercase());
        let res: String = conn.get(&key)?;
        let val: Self = serde_json::from_str(&res)?;
        Ok(val)
    }

    /// Write the AppUser data to storage.
    pub fn save(&self) -> AppResult<()> {
        let mut conn = get_redis_conn(); // TODO: use connection pool
        let val = serde_json::to_string(&self)?;
        let key = format!("rmrs-user-{}", &self.name.to_ascii_lowercase());
        let _res: String = conn.set(&key, &val)?;
        Ok(())
    }

    /// Load the user's subreddit subscriptions and attach them to the user object.
    pub fn load_subs(&self) -> AppResult<Self> {
        let mut conn = get_redis_conn(); // TODO: use connection pool
        let key = format!("user-{}-subs", self.name);
        let raw: String = conn.get(key)?;
        println!(">>> load_subscriptions raw string: {:?}", &raw);
        // AppUserSubscription serde_json::from_str(&raw)?

        Ok(Self { ..Default::default() })
    }

    /// Write only the list of user subscriptions to storage.
    fn save_subs(&self) -> AppResult<()> {
        let mut conn = get_redis_conn(); // TODO: use connection pool
        let key = format!("user-{}-subs", self.name);
        let _: () = conn.set(key, serde_json::to_string(&self.subs)?)?;

        Ok(())
    }
}
