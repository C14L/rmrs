#![allow(dead_code)]

/// Endpoints accessible on the /api route.
// extern crate redis;

use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::jwt;

pub struct Sr {
    id: String,           // sans "t5_"
    display_name: String, // primary name to index
    name: Option<String>,
    created: Option<String>, // created_utc
    url: Option<String>,     // max_length=50; e.g. "/r/de"
    over18: Option<bool>,
    lang: Option<String>,           // max_length=10; language
    title: Option<String>,          // max_length=100
    header_title: Option<String>,   // max_length=100
    subreddit_type: Option<String>, // "public", "restricted", or "private"
    subscribers: Option<u64>,       // subreddit subscribers count
    subscribers_here: Option<u64>,  // subreddit subscribers with an account on reddmeet
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPic {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSubs {
    sr: String,
    is_contributor: Option<bool>,
    is_moderator: Option<bool>,
    is_subscriber: Option<bool>,
    is_banned: Option<bool>,
    is_muted: Option<bool>,
    is_favorite: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    pics: Vec<UserPic>,  //
    subs: Vec<UserSubs>, // user-1-subs
}

fn get_redis_conn() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let conn = client.get_connection().unwrap();
    conn
}

fn fetch_sr_subscribed(user_id: u32) -> Vec<UserSubs> {
    let mut conn = get_redis_conn(); // TODO: use connection pool
    let key = format!("user-{}-subs", user_id);
    let raw: String = conn.get(key).unwrap_or_default();
    serde_json::from_str(&raw).unwrap_or_default()
}

fn save_sr_subscribed(user_id: u32, data: Vec<UserSubs>) {
    let mut conn = get_redis_conn(); // TODO: use connection pool
    let key = format!("user-{}-subs", user_id);

    let _: () = conn
        .set(key, serde_json::to_string(&data).unwrap())
        .unwrap();
}

pub fn srlist_get(info: web::Path<(u32,)>) -> Result<HttpResponse> {
    let srlist = fetch_sr_subscribed(info.0);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(format!("{}", serde_json::to_string(&srlist)?)))
}

pub fn srlist_post(info: web::Path<(u32,)>) -> Result<HttpResponse> {
    let _subs = fetch_sr_subscribed(info.0);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body("[]"))
}

pub fn pics_get(_info: web::Path<(u32,)>) -> Result<HttpResponse> {
    let mut conn = get_redis_conn(); // TODO: use connection pool
    let result: Vec<String> = conn.lrange("p1", 0, -1).unwrap();
    let pics: Vec<UserPic> = result.iter().map(|x| UserPic { url: x.clone() }).collect();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(format!("{}", serde_json::to_string(&pics)?)))
}

pub fn pics_post(_info: web::Path<(u32,)>) -> Result<HttpResponse> {
    // (msg: Json<UserPic>) -> JsonValue {
    // let conn = get_redis_conn();  // TODO: use connection pool
    // let _: usize = conn.rpush("p1", &msg.url).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body("[]"))
}

// Route: /api/init.json
// The JWT is in header field "Authorization: Bearer abc123...def456"
// This is the initial call after the SPA is loaded. The SPA will take
// the JWT either from the URL path's "x" patameter, or load it from
// the browser's LocalStorage. The JWT always contains the user's
// username.
pub fn init_get(req: HttpRequest) -> Result<HttpResponse> {
    println!(">>> init_get request...");
    let header = match req.headers().get("Authorization") {
        Some(x) => match x.to_str() {
            Ok(y) => y[7..].to_string(),  // strip the "Bearer " from the begining
            Err(e) => {
                println!(">>> ERROR init_get (JWT has no content): {:?}", e);
                return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).content_type("application/json; charset=utf-8").body("JWT has no content"));
            },
        },
        None => {
            println!(">>> ERROR init_get (No JWT in header)");
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).content_type("application/json; charset=utf-8").body("No JWT in header"));
        },
    };
    println!(">>> init_get got header: {:?}", header);
    let jwt_token = match jwt::JwtTokenToken::from_string(&header) {
        Ok(x) => x,
        Err(e) => {
            println!(">>> ERROR init_get (JWT invalid): {:?}", e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).content_type("application/json; charset=utf-8").body(format!("JWT invalid: {}", e)));
        },
    };
    println!(">>> init_get got jwt_token: {:?}", jwt_token);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body("[]"))
}
