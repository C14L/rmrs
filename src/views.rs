//
// Each function is a view/controller for a HTTP route,
// takes a HttpRequest and returns a HttpResponse.
//

extern crate reqwest;

use actix_web;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// use crate::helpers;
use crate::helpers::User;

const REDDIT_TOKEN_URL: &'static str = "https://www.reddit.com/api/v1/access_token";
const APP_OAUTH_CB: &'static str = "http://localhost:8001/redditcallback.html";
const APP_NAME: &'static str = "BtNjVhBUlLJDXg";
const APP_SECRET: &'static str = "i5x4WPmHUA6Q7rYHB1SuOMemgSs";
const APP_USER_AGENT: &'static str = "web:reddmeet.com:v2.0.0a/rust (by /u/C14L)";

pub fn home(req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    println!("> > >  NEW home REQUEST  < < <");
    match User::from(&req) {
        Some(u) => Ok(format!("Hello, you are {}!", u.username).into()),
        None => Ok(format!("Hey, you need <a href=/>to authenticate</a> first!").into()),
    }
}

// Do not use session cookie: keep all session data on the client!
pub fn redditauth(_req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    println!("> > >  NEW redditauth REQUEST  < < <");
    let state = String::from("kfhldhdgdfggeruhnstringlgp09u40hor"); // TODO: randomize
    let url = format!("https://ssl.reddit.com/api/v1/authorize?\
        duration=permanent&\
        response_type=code&\
        scope=identity,mysubreddits&\
        redirect_uri={}&\
        state={}&\
        client_id={}", APP_OAUTH_CB, state, APP_NAME);
    Ok(actix_web::HttpResponse::Found().header(actix_web::http::header::LOCATION, url).finish())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAuthCallback {
    state: String,
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct RedditAccessToken {
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<usize>,
    refresh_token: Option<String>,
    scope: Option<String>,
    update_time: Option<u64>,
    expire_time: Option<u64>,
    create_time: Option<u64>,
}

impl RedditAccessToken {

    pub fn new(code: &String) -> Option<RedditAccessToken> {
        println!(">>> RedditAccessToken::new() with code: {:?}", code);

        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", APP_OAUTH_CB),
        ];

        println!(">>> body: {:?}", &body);
        println!(">>> url: {:?}", &url);

        let req = reqwest::Client::new().post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body);

        println!(">>> req: {:?}", &req);

        let res = req.send().unwrap_or_else(|e| {
            println!(">>> Well, not really. (1)");
            return None
        });

        res.json().and_then(|mut x: RedditAccessToken| {
            println!(">>> RedditAccessToken response: {:?}", &x);
            let t = SystemTime::now().duration_since(UNIX_EPOCH).expect("No time?").as_secs();
            x.create_time = Some(t);
            x.update_time = Some(t);
            x.expire_time = x.expires_in.map(|x| x as u64 + t);
            Ok(x)
        }).or_else(|e| {
            println!(">>> Well, not really. (2)");
            Err(e)
        }).ok()
    }

    // pub fn refresh(self) -> Result<RedditAccessToken> {
    //     let body = [
    //         ("grant_type", "refresh_token"),
    //         ("refresh_token", &self.refresh_token.expect("No token!").as_str()),
    //     ];
    //     reqwest::Client::new()
    //         .post(Url::parse(REDDIT_TOKEN_URL).unwrap())
    //         .basic_auth(APP_NAME, Some(APP_SECRET))
    //         .form(&body)
    //         .send()?
    //         .json()
    //         .and_then(|mut x: RedditAccessToken| {
    //             let t = SystemTime::now()
    //                 .duration_since(UNIX_EPOCH)
    //                 .expect("No time?")
    //                 .as_secs();
    //             x.update_time = Some(t);
    //             x.expire_time = x.expires_in.map(|x| x as u64 + t);
    //             Ok(x)
    //         })
    // }
}

// After Reddit auth page: check state and use code to get first token.
pub fn redditcallback(
    _req: actix_web::HttpRequest,
    params: actix_web::web::Query<RedditAuthCallback>,
) -> actix_web::Result<actix_web::HttpResponse> {

    println!("> > >  NEW redditcallback REQUEST  < < <");
    println!(">>> code {:?}", &params.code);
    println!(">>> state {:?}", &params.state);

    // let token: Option<RedditAccessToken> = RedditAccessToken::new(&params.code);
    let token = Some(format!("Hallo!"));

    println!(">>> token {:?}", &token);

    // Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
    //     .content_type("text/html; charset=utf-8")
    //     .body(format!("Reddit auth done:"))
    // )

    match token {
        Some(t) => Ok(format!("Reddit auth successful.").into()),
        None => Ok(format!("Reddit auth failed :(").into()),
    }
}
