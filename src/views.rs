//
// Each function is a view/controller for a HTTP route,
// takes a HttpRequest and returns a HttpResponse.
//

extern crate jsonwebtoken;
extern crate reqwest;
extern crate serde_derive;

use actix_web;
use futures::future::Future;
use jsonwebtoken as jwt;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

// use crate::helpers;
use crate::helpers;
use crate::models;

const REDDIT_TOKEN_URL: &'static str = "https://www.reddit.com/api/v1/access_token";
const APP_OAUTH_CB: &'static str = "http://localhost:8001/redditcallback.html";
const APP_NAME: &'static str = "BtNjVhBUlLJDXg";
const APP_SECRET: &'static str = "i5x4WPmHUA6Q7rYHB1SuOMemgSs";
const APP_USER_AGENT: &'static str = "web:reddmeet.com:v2.0.0a/rust (by /u/C14L)";
const APP_JWT_SECRET: &'static str = "io679oyry9y6@*YO(*Y(Y9ogout6od9890@&(&@!!NNDLK'>.OIJI@(JGHKBXM<";

pub fn testing(
    _req: actix_web::HttpRequest,
) -> impl Future<Item = actix_web::HttpResponse, Error = actix_web::Error> {
    println!(">>> testing request begin");
    let url = Url::parse("https://example.com/").unwrap();
    println!(">>> testing url built");
    let req = reqwest::Client::new().get(url);
    println!(">>> testing req prepared");
    actix_web::web::block(move || req.send())
        .from_err()
        .and_then(|res| {
            println!(">>> testing res: {:?}", &res);
            actix_web::HttpResponse::Ok()
                .content_type("text/html")
                .body("Hello!")
        })
}

pub fn home(req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    println!(">>> New home request.");
    match models::User::from(&req) {
        Some(u) => Ok(format!("Hello, you are {}!", u.username).into()),
        None => {
            let mut file = File::open("../frontend/index.html")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            Ok(
                actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
                    .content_type("text/html; charset=utf-8")
                    .body(contents),
            )
        }
    }
}

// Do not use session cookie: keep all session data on the client!
pub fn redditauth(_req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    println!(">>> New redditauth request, redirecting...");
    let state = Uuid::new_v4().to_string(); //String::from("kfhldhdgdfggeruhnstringlgp09u40hor"); // TODO: randomize
    let url = format!(
        "https://ssl.reddit.com/api/v1/authorize?\
         duration=permanent&\
         response_type=code&\
         scope=identity,mysubreddits&\
         redirect_uri={}&\
         state={}&\
         client_id={}",
        APP_OAUTH_CB, state, APP_NAME
    );
    Ok(actix_web::HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, url)
        .finish())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAuthCallback {
    state: String,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct JwtTokenToken {
    access_token: String,
    refresh_token: String,
    username: String,
    user_id: u64,
    expire_time: u64,
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

        println!(">>> RedditAccessToken::new() --> body: {:?}", &body);
        println!(">>> url: {:?}", &url);

        let req = reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body);

        println!(">>> req: {:?}", &req);

        let mut res = req.send().unwrap();

        res.json()
            .and_then(|mut x: RedditAccessToken| {
                println!(">>> RedditAccessToken response: {:?}", &x);
                let t = helpers::unix_timestamp().expect("No time?");
                x.create_time = Some(t);
                x.update_time = Some(t);
                x.expire_time = x.expires_in.map(|x| x as u64 + t);
                Ok(x)
            })
            .or_else(|e| {
                println!(">>> Well, not really. (2)");
                Err(e)
            })
            .ok()
    }

    // pub fn refresh(self) -> Result<RedditAccessToken> {
    //    let url = Url::parse(REDDIT_TOKEN_URL).unwrap()
    //     let body = [
    //         ("grant_type", "refresh_token"),
    //         ("refresh_token", &self.refresh_token.expect("No token!").as_str()),
    //     ];
    //     reqwest::Client::new().post(url)
    //         .basic_auth(APP_NAME, Some(APP_SECRET))
    //         .form(&body)
    //         .send()?
    //         .json()
    //         .and_then(|mut x: RedditAccessToken| {
    //             let t = helpers::unix_timestamp().expect("No time?");
    //             x.update_time = Some(t);
    //             x.expire_time = x.expires_in.map(|x| x as u64 + t);
    //             Ok(x)
    //         })
    // }
}

impl JwtTokenToken {
    pub fn from(user: &models::User, reddit_token: &RedditAccessToken) -> Result<String, Box<dyn Error>> {
        let jwt_header = jwt::Header::default();
        let jwt_claims = JwtTokenToken {
            refresh_token: reddit_token
                .refresh_token
                .to_owned()
                .expect("No refresh token!"),
            access_token: reddit_token
                .access_token
                .to_owned()
                .expect("No access token!"),
            username: user.username.to_owned(),
            user_id: user.id.to_owned(),
            expire_time: helpers::unix_timestamp().expect("No time?"),
        };
        jwt::encode(&jwt_header, &jwt_claims, APP_JWT_SECRET.as_ref()).map_err(|e| e.into())
    }
}

// After Reddit auth page: check state and use code to get first token.
pub fn redditcallback(
    req: actix_web::HttpRequest,
    params: actix_web::web::Query<RedditAuthCallback>,
)
    -> actix_web::Result<actix_web::HttpResponse>
{
    println!(">>> New redditcallback request: code {:?} / state {:?}", &params.code, &params.state);
    let reddit_token: RedditAccessToken = RedditAccessToken::new(&params.code).unwrap();
    // TODO: with the reddit_token, fetch basic user data (username, etc) from reddit to put in JWT.
    let user = models::User::from(&req).unwrap_or_default();
    let jwt_token = JwtTokenToken::from(&user, &reddit_token).unwrap();
    let url = format!("/home#x={}", &jwt_token);
    Ok(actix_web::HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, url)
        .finish())
}

// After redditcallback, we
pub fn app(_req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    let mut file = File::open("../frontend/app.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(
        actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(contents),
    )
}
