#![allow(dead_code, unused_imports)]

/// Endpoints accessible on the /api route.

use crate::helpers::AppResult;
use actix_web::http::HeaderMap;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use redis::{Commands, Connection};

use crate::jwt::JwtTokenToken;
use crate::models::app_user::AppUser;
use crate::models::app_pushtoken::AppPushtoken;

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
    let header = match headers.get("Authorization") {
        Some(x) => match x.to_str() {
            Ok(y) => y[7..].to_string(), // strip the "Bearer " from the begining
            Err(_) => return Err("JWT has no content.".into()),
        },
        None => return Err("No JWT in header.".into()),
    };

    JwtTokenToken::from_string(&header)
}

/// Route: /api/user/me.json
///
/// The JWT is in header field "Authorization: Bearer abc123...def456"
/// This is the initial call after the SPA is loaded. The SPA will take
/// the JWT either from the URL path's "x" patameter, or load it from
/// the browser's LocalStorage. The JWT always contains the user's
/// username.
pub fn user_me_get(req: HttpRequest) -> ActixResult<HttpResponse> {
    let token = get_token_from_header(&req.headers()).unwrap(); // TODO: handle error

    let _ =  AppPushtoken::fetch().ok();

    match AppUser::load(&token.username) {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
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
