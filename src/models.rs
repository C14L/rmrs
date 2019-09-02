// Data models for this app
//
#![allow(dead_code)]

use crate::jwt;
use crate::redditapi;

#[derive(Default)]
pub struct User {
    pub created: f64,
    pub name: String,
}

impl User {
    pub fn from_reddit(reddit_user: &redditapi::UserBasics) -> Option<Self> {
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

