/// Authenticate to access Reddit API endpoints.
/// https://github.com/reddit-archive/reddit/wiki/OAuth2

extern crate reqwest;

use reqwest::Url;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::content::Html;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use serde::Deserialize;
//use std::collections::HashMap;

const REDDIT_API_ENDPOINT: &'static str = "https://www.reddit.com/api/v1/access_token";
const API_ENDPOINT: &'static str = "http://localhost:8001/redditcallback.html";
const APP_NAME: &'static str = "BtNjVhBUlLJDXg";
const APP_SECRET: &'static str = "i5x4WPmHUA6Q7rYHB1SuOMemgSs";

#[derive(FromForm)]
pub struct RedditCallbackReply {
    error: Option<String>,
    code: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RedditAccessToken {
    access_token: String,
    token_type: String,
    expires_in: usize,
    refresh_token: String,
    scope: String,
}
// {
//     "access_token": "46597193-8-RsLVzf98n3OJFNZEXXXfFJddU",
//     "token_type": "bearer",
//     "expires_in": 3600,
//     "refresh_token": "46597193-pu-dfgzJByDgp_qCvXXX6hAlTtE",
//     "scope": "identity"
// }

impl RedditAccessToken {

    pub fn new(code: String) -> Result<RedditAccessToken, reqwest::Error> {
        println!("THIS IS RedditAccessToken::new()");
        let body = [
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", "http://localhost:8001/redditcallback.html"),
        ];
        reqwest::Client::new()
            .post(Url::parse(REDDIT_API_ENDPOINT).unwrap())
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .form(&body)
            .send()?
            .json()
    }

    pub fn refresh(self) -> Result<RedditAccessToken, reqwest::Error> {
        let body = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token.as_str()),
        ];
        reqwest::Client::new()
            .post(Url::parse(REDDIT_API_ENDPOINT).unwrap())
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .form(&body)
            .send()?
            .json()
    }
}

// Simply redirects the client to Reddit's oAuth page.
#[get("/redditauth.html")]
pub fn oauth_call_get() -> Result<Redirect, NotFound<String>> {
    // TODO: Generate a `state` uuid and remember it to verify in oauth_callback.
    // TODO: Load `client_id` and `redirect_uri` from some conf file.
    Ok(Redirect::to(format!(
        "https://ssl.reddit.com/api/v1/authorize?\
         duration=permanent&response_type=code&scope=identity&\
         redirect_uri={}&state=kfhldhdgdfggeruhnstringlgp09u40hor&\
         client_id={}", API_ENDPOINT, APP_NAME
    )))
}

#[get("/redditcallback.html?<params..>")]
pub fn oauth_callback_get(
    mut cookies: Cookies,
    params: Form<RedditCallbackReply>,
) -> Result<Html<String>, NotFound<String>> {
    println!("THIS IS: oauth_callback_get()");

    let sess_cookie = match cookies.get_private("rmrs_sessid") {
        Some(c) => c,
        None => {
            // TODO: Generate a cookie uuid here.
            let c = Cookie::new("rmrs_sessid", "a8yr7ersdfsd4iuyhli54-=09u8hfi3;fdk-0");
            cookies.add_private(c.clone());
            c
        }
    };
    println!("COOKIE: {:?}", &sess_cookie);

    let token: Result<RedditAccessToken, reqwest::Error> = match &params.code {
        Some(code) => RedditAccessToken::new(code.to_string()),
        None => return Err(NotFound("Invalid code.".to_string())),
    };
    println!("TOKEN RECEIVED: {:?}", &token);

    match token {
        Ok(t) => {
            Ok(Html(format!(
                "cookie: '{}' and state: '{}' -- access_token: '{}', token_type: '{}', expires_in: '{}', scope: '{}', refresh_token: '{}'.",
                sess_cookie,
                params.state.clone().unwrap_or_default(),
                t.access_token,
                t.token_type,
                t.expires_in,
                t.scope,
                t.refresh_token,
            )))
        },
        Err(_) => Err(NotFound("Invalid token.".to_string()))
    }
}
