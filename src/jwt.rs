extern crate jsonwebtoken;

use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::helpers;
use crate::models;
use crate::redditapi;

const APP_JWT_SECRET: &'static str = "io679oyry9y6@*YO(*Y(Y9ogout6od9890@&(&@!!NNDLK'>.OIJI@(JGHKBXM<";
const TOKEN_DEFAUT_LIFETIME_SECS: u64 = 3600;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtTokenToken {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub created: f64,
    pub exp: u64,
}

impl JwtTokenToken {
    pub fn new(user: &models::User, reddit_token: &redditapi::RedditAccessToken) -> Self {
        JwtTokenToken {
            refresh_token: reddit_token.refresh_token.as_ref().unwrap().to_owned(),
            access_token: reddit_token.access_token.as_ref().unwrap().to_owned(),
            username: user.name.to_owned(),
            created: user.created.to_owned(),
            exp: helpers::unix_timestamp().expect("No time?") + TOKEN_DEFAUT_LIFETIME_SECS,
        }
    }

    pub fn from_string(token: &String) -> Result<Self, Box<dyn Error>> {
        let secret = APP_JWT_SECRET.as_bytes();
        let token_data = jwt::decode::<JwtTokenToken>(&token, secret, &jwt::Validation::new(jwt::Algorithm::HS256));
        token_data.map(|x| x.claims).map_err(|e| e.into())
    }

    pub fn to_string(self) -> Result<String, Box<dyn Error>> {
        let secret = APP_JWT_SECRET.as_bytes();
        let jwt_header = jwt::Header::default();
        jwt::encode(&jwt_header, &self, secret.as_ref()).map_err(|e| e.into())
    }
}
