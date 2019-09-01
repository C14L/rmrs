// Data models for this app
//

use crate::jwt;
use crate::redditapi;

#[derive(Default)]
pub struct User {
    pub id: u64,
    pub username: String,
}

impl User {
    pub fn from_reddit(_reddit_user: &redditapi::UserBasics) -> Option<Self> {
        None
        // Some(Self {
        //     id: 123,
        //     username: String::from("abc123"),
        // })
    }

    pub fn from_jwt(token: &jwt::JwtTokenToken) -> Option<Self> {
        Some(Self {
            id: token.user_id.to_owned(),
            username: token.username.to_owned(),
        })
    }
}
