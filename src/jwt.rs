extern crate jsonwebtoken;

use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::conf::{APP_JWT_SECRET, TOKEN_DEFAUT_LIFETIME_SECS};
use crate::helpers;
use crate::models::app_user::AppUser;
use crate::models::reddit_token::RedditToken;

pub type JwtResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtTokenToken {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub created: f64,
    pub exp: u64,
}

impl JwtTokenToken {
    pub fn new(user: &AppUser, reddit_token: &RedditToken) -> JwtResult<Self> {
        Ok(Self {
            access_token: reddit_token.access_token.to_owned(),
            refresh_token: reddit_token.refresh_token.to_owned(),
            username: user.name.to_owned(),
            created: user.created.to_owned(),
            exp: helpers::unix_timestamp_in_secs_from_now(TOKEN_DEFAUT_LIFETIME_SECS)?,
        })
    }

    pub fn from_string(token: &String) -> JwtResult<Self> {
        let secret = APP_JWT_SECRET.as_bytes();
        println!("@@@ JwtTokenToken::from_string() -> secret: {:?}", &secret);
        let alg = jwt::Validation::new(jwt::Algorithm::HS256);
        println!("@@@ JwtTokenToken::from_string() -> alg: {:?}", &alg);
        let token_data = jwt::decode::<JwtTokenToken>(&token, secret, &alg);
        println!("@@@ JwtTokenToken::from_string() -> token_data: {:?}", &token_data);
        token_data.map(|x| x.claims).map_err(|e| e.into())
    }

    pub fn to_string(self) -> JwtResult<String> {
        let secret = APP_JWT_SECRET.as_bytes();
        let jwt_header = jwt::Header::default();
        jwt::encode(&jwt_header, &self, secret.as_ref()).map_err(|e| e.into())
    }

    pub fn refresh(self, reddit_token: &RedditToken) -> JwtResult<Self> {
        Ok(Self {
            access_token: reddit_token.access_token.to_owned(),  // new
            refresh_token: self.refresh_token.to_owned(),
            username: self.username.to_owned(),
            created: self.created.to_owned(),
            exp: helpers::unix_timestamp_in_secs_from_now(TOKEN_DEFAUT_LIFETIME_SECS)?,
        })
    }
}
