
use reqwest::Url;
use serde::{Serialize, Deserialize};
use reqwest::header::USER_AGENT;

use crate::conf::{APP_OAUTH_CB, REDDIT_TOKEN_URL, APP_SECRET, APP_NAME, APP_USER_AGENT};
use crate::helpers::unix_timestamp;

pub fn get_reddit_authorize_url(state: String) -> String {
    format!("https://ssl.reddit.com/api/v1/authorize?duration=permanent&response_type=code&\
        scope=identity,mysubreddits&redirect_uri={}&state={}&client_id={}",
        APP_OAUTH_CB, state, APP_NAME)
}

/// Redit Auth

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAuthCallback {
    pub state: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAccessToken {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<usize>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub update_time: Option<u64>,
    pub expire_time: Option<u64>,
    pub create_time: Option<u64>,
}

impl RedditAccessToken {
    pub fn new(code: &String) -> Option<Self> {
        println!(">>> RedditAccessToken::new() with code: {:?}", code);

        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", APP_OAUTH_CB),
        ];
        println!(">>> RedditAccessToken::new() --> body: {:?}", &body);
        println!(">>> RedditAccessToken --> url: {:?}", &url);
        let req = reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body);
        println!(">>> RedditAccessToken --> req: {:?}", &req);
        let mut res = req.send().unwrap();  // TODO: catch possible error!!!
        println!(">>> RedditAccessToken --> res: {:?}", &res);
        let json = res.json();
        println!(">>> RedditAccessToken --> json: {:?}", &json);
        json.and_then(|mut x: RedditAccessToken| {
            println!(">>> RedditAccessToken --> unmarshalled response: {:?}", &x);
            let t = unix_timestamp().expect("No time?");
            x.create_time = Some(t);
            x.update_time = Some(t);
            x.expire_time = Some(x.expires_in.unwrap() as u64 + t);
            Ok(x)
        })
        .or_else(|e| {
            println!(">>> RedditAccessToken --> ERROR unmarshalling response: {:?}", &e);
            Err(e)
        }).ok()
    }

    pub fn refresh(&mut self) -> Option<&Self> {
        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token.to_owned().unwrap()),
        ];

        let req = reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body);
        println!(">>> RedditAccessToken --> req: {:?}", &req);
        let mut res = req.send().unwrap();  // TODO: catch possible error!!!
        println!(">>> RedditAccessToken --> res: {:?}", &res);
        let json: Result<RedditAccessToken, _> = res.json();

        match json {
            Ok(x) => {
                println!(">>> RedditAccessToken --> json: {:?}", &x);
                let t = unix_timestamp().expect("No time?");
                self.expires_in = x.expires_in;
                self.refresh_token = x.refresh_token;
                self.update_time = Some(t);
                self.expire_time = Some(x.expires_in.unwrap() as u64 + t);
                Some(self)
            },
            Err(_) => None,
        }
    }
}
