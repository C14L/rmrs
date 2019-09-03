// Data models for this app
//
#![allow(dead_code)]

use crate::jwt;
use crate::models;

#[derive(Default)]
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
}
