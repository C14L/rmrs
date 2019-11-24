use crate::jwt::JwtTokenToken;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::conf::{APP_NAME, APP_OAUTH_CB, APP_SECRET, APP_USER_AGENT, REDDIT_TOKEN_URL};
use crate::helpers::{unix_timestamp, AppResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAuthCallback {
    pub state: String,
    pub code: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RedditToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,

    #[serde(default)]
    pub create_time: u64,

    #[serde(default)]
    pub update_time: u64,

    #[serde(default)]
    pub expire_time: u64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct RedditRefreshToken {
    access_token: String, // Your access token
    token_type: String,   // The word "bearer"
    expires_in: usize,    // Unix Epoch Seconds
    scope: String,        // A scope string
}

impl RedditToken {
    /// Using the state value, it builds the URL to redirect the user to for authorization.
    pub fn get_auth_url(state: String) -> String {
        format!(
            "https://ssl.reddit.com/api/v1/authorize?duration=permanent&response_type=code&\
             scope=identity,mysubreddits&redirect_uri={}&state={}&client_id={}",
            APP_OAUTH_CB, state, APP_NAME
        )
    }

    /// Extract the token strings from a JWT.
    pub fn from_jwt(jwt: &JwtTokenToken) -> AppResult<Self> {
        Ok(Self {
            access_token: jwt.access_token.to_owned(),
            refresh_token: jwt.refresh_token.to_owned(),
            ..Default::default()
        })
    }

    pub fn new(code: &String) -> AppResult<Self> {
        let url = Url::parse(REDDIT_TOKEN_URL)?;
        let body = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", APP_OAUTH_CB),
        ];

        println!("RedditToken::new() will send request with body: {:?}", &body);

        reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body)
            .send()?
            .json()
            .map(|mut x: RedditToken| {
                let t = unix_timestamp().expect("No time?");
                x.create_time = t;
                x.update_time = t;
                x.expire_time = x.expires_in as u64 + t;
                x
            })
            .map_err(|e| e.into())
    }

    pub fn refresh(&mut self) -> AppResult<()> {
        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token.to_owned()),
        ];
        let _ = reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body)
            .send()?
            .json()
            .map(|x: RedditRefreshToken| {
                let t = unix_timestamp().expect("No time?");
                self.access_token = x.access_token;
                self.expires_in = x.expires_in;
                self.expire_time = x.expires_in as u64 + t; // access token expire, NOT jwt expire
                self.update_time = t;
            });

        Ok(())
    }

    /// Fetch data that requires token authentication from Reddit API.
    ///
    /// TODO: If the access token is expired, this method will automatically
    /// attempt to renew the token and re-try the request.
    pub fn fetch<T>(&mut self, url: &'static str) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        let url = Url::parse(url)?;
        let query_params = [("raw_json", "1")];
        let req = reqwest::Client::new()
            .get(url.to_owned())
            .query(&query_params)
            .header(USER_AGENT, APP_USER_AGENT)
            .header("Authorization", format!("bearer {}", &self.access_token));

        let data = match req.send() {
            Ok(val) => {
                if let Some(auth) = &val.headers().get("www-authenticate") {
                    if auth.to_str()?.contains("invalid_token") {
                        println!("### INVALID TOKEN! --> {:?}", &auth);
                        println!("### TOKEN BEFORE --> {:?}", self.access_token);
                        self.refresh()?;
                        println!("### TOKEN AFTER --> {:?}", self.access_token);

                        let req2 = reqwest::Client::new()
                            .get(url.to_owned())
                            .query(&query_params)
                            .header(USER_AGENT, APP_USER_AGENT)
                            .header("Authorization", format!("bearer {}", self.access_token));

                        match req2.send() {
                            Ok(val) => Ok(val),
                            Err(e) => Err(e),
                        }
                    } else {
                        Ok(val)
                    }
                } else {
                    Ok(val)
                }
            }
            Err(e) => {
                println!("### RedditToken::fetch ERR-01 --> {:?}", &e);
                Err(e)
            }
        };

        match data {
            Ok(mut val) => match val.json() {
                Ok(d) => Ok(d),
                Err(e) => {
                    println!("### RedditToken::fetch ERR-02 --> {:?}", &e);
                    Err(e.into())
                }
            },
            Err(e) => {
                    println!("### RedditToken::fetch ERR-03 --> {:?}", &e);
                    panic!("BLah!");
            }
        }
    }
}
