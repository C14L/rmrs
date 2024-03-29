#![allow(dead_code, unused_imports)]

use actix_web::http::header::AUTHORIZATION;
use actix_web::http::HeaderMap;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use redis::{Commands, Connection};

use crate::helpers::AppResult;
use crate::jwt::JwtTokenToken;
use crate::models::app_user::AppUser;
use crate::models::app_subreddit::AppSubreddit;
use crate::models::reddit_user::RedditUser;
use crate::models::reddit_user_subreddit::RedditUserSubredditList;
use crate::models::reddit_user_subreddit_karma::RedditUserSubredditKarmaList;
use crate::models::reddit_token::RedditToken;


const CONTENT_TYPE: &'static str = "application/json; charset=utf-8";

fn short_json(status: StatusCode, msg: &'static str) -> ActixResult<HttpResponse> {
    Ok(HttpResponse::build(status)
        .content_type(CONTENT_TYPE)
        .body(format!("[\"msg\": \"{}\"]", msg)))
}

fn get_redis_conn() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let conn = client.get_connection().unwrap();
    conn
}

/// Toggle activation state of a subreddit subscription in a user profile.
pub fn user_me_subs_post() -> ActixResult<HttpResponse> {
    short_json(StatusCode::OK, "Not implemented.")
}

/// Add a new picture link to a user profile.
pub fn user_me_pics_post() -> ActixResult<HttpResponse> {
    short_json(StatusCode::OK, "Not implemented.")
}

fn get_token_from_header(headers: &HeaderMap) -> AppResult<JwtTokenToken> {
    let b64token = match headers.get("Authorization") {
        Some(x) => match x.to_str() {
            Ok(y) => y[7..].to_string(),
            Err(_) => return Err("JWT has no content.".into()),
        },
        None => return Err("No JWT in header.".into()),
    };
    println!("@@@ get_token_from_header() -> b64token: {:?}", &b64token);
    JwtTokenToken::from_string(&b64token)
}

/// Route: /api/user/me.json
///
/// The JWT is in header field "Authorization: Bearer abc123...def456"
/// This is the initial call after the SPA is loaded. The SPA will take
/// the JWT either from the URL path's "x" patameter, or load it from
/// the browser's LocalStorage. The JWT always contains the user's
/// username.
pub fn user_me_get(req: HttpRequest) -> ActixResult<HttpResponse> {
    println!("####################################################");
    println!("### user_me_get --> init");
    let headers = &req.headers();
    println!("####################################################");
    println!("### user_me_get --> got headers: {:?}", &headers);
    let jwt_token = match get_token_from_header(headers) {
        Ok(x) => x,
        Err(e) => {
            // TODO: handle error
            println!("### user_me_get --> ERROR unwrap get_token_from_header(headers): {:?}", e);
            return short_json(StatusCode::OK, "ERROR unwrap get_token_from_header(headers).")
        },
    };
    println!("####################################################");
    println!("### user_me_get --> got jwt_token: {:?}", &jwt_token);
    let reddit_token = &mut RedditToken::from_jwt(&jwt_token).unwrap(); // TODO: handle error
    println!("####################################################");
    println!("### user_me_get --> got reddit_token: {:?}", &reddit_token);
    let res = RedditUser::fetch_me(reddit_token);
    println!("####################################################");
    println!("### user_me_get --> got reddit user: {:?}", &res);
    let jwt_token = jwt_token.refresh(&reddit_token).unwrap();
    println!("####################################################");
    println!("### user_me_get --> refreshed jwt_token: {:?}", &jwt_token);

    match AppUser::load(&jwt_token.username) {
        Ok(user) => {
            println!("####################################################");
            println!("### user_me_get --> AppUser loaded: {:?}", &user);
            let t = jwt_token.to_string().unwrap();
            println!("####################################################");
            println!("### user_me_get --> AppUser jwt_token: {:?}", &t);
            Ok(HttpResponse::Ok().header(AUTHORIZATION, t).json(&user))
        },
        Err(_) => return short_json(StatusCode::NOT_FOUND, "User not found."),
    }
}

pub fn user_me_post() -> ActixResult<HttpResponse> {
    short_json(StatusCode::OK, "Not implemented.")
}

/// Route: /api/search.json
///
/// The (cached) results of a search for Reditors with similar subreddit
/// subscriptions. The individual search parameters are set via POST
/// elsewhere, like the list of subs or the geo location.
pub fn search_get() -> ActixResult<HttpResponse> {
    short_json(StatusCode::OK, "Not implemented.")
}
