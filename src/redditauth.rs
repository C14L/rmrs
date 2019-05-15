/// Authenticate to access Reddit API endpoints.
/// https://github.com/reddit-archive/reddit/wiki/OAuth2

extern crate reqwest;

use log::debug;
use reqwest::Url;
use rocket::http::{Cookie, Cookies, Status};
use rocket::response::content::Html;
use rocket::request::Form;
use serde::Deserialize;

#[derive(FromForm)]
pub struct RedditCallbackReply {
    error: Option<String>,
    code: Option<String>,
    state: String,
}

#[derive(Deserialize)]
pub struct RedditAccessToken {
    token: String,
}

pub struct RedditAccessTokenRequestBody {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

impl RedditAccessToken {
    pub fn from_code(code: String) -> Result<RedditAccessToken, reqwest::Error> {
        let url = Url::parse("https://www.reddit.com/api/v1/access_token").unwrap();
        let body = RedditAccessTokenRequestBody {
            grant_type: String::from("authorization_code"),
            code: code,
            redirect_uri: String::from("http://localhost:8001/"),
        };
        // Post request to reddit using `code` value.
        let response: RedditAccessToken = match reqwest::get(url) {
            Ok(mut res1) => {
                match res1.json() {
                    Ok(res2) => res2,
                    Err(err) => RedditAccessToken {
                        token: String::from("No token received.")
                    },
                }
            }
            Err(err) => RedditAccessToken {
                token: String::from("Received an invalid token.")
            }
        };
        debug!("Got a response: {}", &response.token);
        Ok(response)
    }

    pub fn from_refresh_token() -> Result<RedditAccessToken, reqwest::Error> {
        // Post request to reddir using `refresh_token` value.
        let response = RedditAccessToken { token: String::from("Blah!") };
        Ok(response)
    }
}

#[get("/redditcallback.html?<params..>")]
pub fn oauth_callback_get(
        mut cookies: Cookies, params: Form<RedditCallbackReply>
) -> Html<String> {
    let mut response;
    let sess_cookie = match cookies.get_private("sessid") {
        Some(c) => c,
        None => {
            let c = Cookie::new("sessid", "a8yr7to9yiy9t84iuyhliog7u09f8u8hfi3;fdk-0");
            cookies.add_private(c.clone());
            c
        }
    };

    if let Some(code) = &params.code {
        let access = RedditAccessToken::from_code(code.to_string()).unwrap();
        response = format!("access '{}' and code '{}' and state '{}' and cookie '{}'.", access.token, code, params.state, sess_cookie);
    } else {
        response = format!("Hello, sessid '{}'!", sess_cookie);
    }

    Html(response)
}
