use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};

use crate::conf::{APP_OAUTH_CB, REDDIT_TOKEN_URL, APP_SECRET, APP_NAME, APP_USER_AGENT};
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

impl RedditToken {
    /// Using the state value, it builds the URL to redirect the user to for authorization.
    pub fn get_auth_url(state: String) -> String {
        format!("https://ssl.reddit.com/api/v1/authorize?duration=permanent&response_type=code&\
            scope=identity,mysubreddits&redirect_uri={}&state={}&client_id={}",
            APP_OAUTH_CB, state, APP_NAME)
    }

    pub fn new(code: &String) -> AppResult<Self> {
        let url = Url::parse(REDDIT_TOKEN_URL)?;
        let body = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", APP_OAUTH_CB),
        ];
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

    pub fn refresh(&mut self) -> AppResult<&mut Self> {
        // TODO: untested.
        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token.to_owned()),
        ];
        reqwest::Client::new()
            .post(url)
            // .basic_auth(APP_NAME, Some(APP_SECRET)) --> needed ????
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body)
            .send()?
            .json()
            .map(|x: RedditToken| {
                println!(">>> Fresh RedditToken --> json: {:?}", &x);
                let t = unix_timestamp().expect("No time?");
                self.expires_in = x.expires_in;
                self.refresh_token = x.refresh_token;
                self.update_time = t;
                self.expire_time = x.expires_in as u64 + t;  // access token expire, NOT jwt expire
                self
            })
            .map_err(|e| e.into())
    }

    /// Fetch data that requires token authentication from Reddit API.
    ///
    /// TODO: If the access token is expired, this method will automatically
    /// attempt to renew the token and re-try the request.
    pub fn fetch<T>(&self, url: &'static str) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        let url = Url::parse(url)?;
        let query_params = [("raw_json", "1")];

        reqwest::Client::new()
            .get(url)
            .query(&query_params)
            .header("Authorization", format!("bearer {}", self.access_token))
            .header(USER_AGENT, APP_USER_AGENT)
            .send()?
            .json()
            .map(|x: T| x)
            .map_err(|e| e.into())
    }
}
