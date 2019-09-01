extern crate jsonwebtoken;

use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::helpers;
use crate::models;
use crate::redditapi;

const APP_JWT_SECRET: &'static str = "io679oyry9y6@*YO(*Y(Y9ogout6od9890@&(&@!!NNDLK'>.OIJI@(JGHKBXM<";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtTokenToken {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub user_id: u64,
    pub expire_time: u64,
}

impl JwtTokenToken {
    pub fn from_string(token: &String) -> Result<Self, Box<dyn Error>> {
        let secret = APP_JWT_SECRET.as_bytes();
        let token_data = jwt::decode::<JwtTokenToken>(&token, secret, &jwt::Validation::new(jwt::Algorithm::HS256));
        token_data.map(|x| x.claims).map_err(|e| e.into())
    }

    pub fn to_string(user: &models::User, reddit_token: &redditapi::RedditAccessToken) -> Result<String, Box<dyn Error>> {
        let secret = APP_JWT_SECRET.as_bytes();
        let jwt_header = jwt::Header::default();
        let jwt_claims = JwtTokenToken {
            refresh_token: reddit_token.refresh_token.as_ref().unwrap().to_owned(),
            access_token: reddit_token.access_token.as_ref().unwrap().to_owned(),
            username: user.username.to_owned(),
            user_id: user.id.to_owned(),
            expire_time: helpers::unix_timestamp().expect("No time?"),
        };
        jwt::encode(&jwt_header, &jwt_claims, secret.as_ref()).map_err(|e| e.into())
    }
}
