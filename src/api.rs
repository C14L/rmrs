/// Endpoints accessible on the /api route.

// extern crate redis;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Result};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json;

pub struct Sr {
    id: String, // sans "t5_"
    display_name: String,           // primary name to index
    name: Option<String>,
    created: Option<String>,        // created_utc
    url: Option<String>,            // max_length=50; e.g. "/r/de"
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
    pics: Vec<UserPic>,   //
    subs: Vec<UserSubs>,  // user-1-subs
}

fn get_redis_conn() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let conn = client.get_connection().unwrap();
    conn
}

fn fetch_sr_subscribed(user_id: u32) -> Vec<UserSubs> {
    let mut conn = get_redis_conn();  // TODO: use connection pool
    let key = format!("user-{}-subs", user_id);
    let raw: String = conn.get(key).unwrap_or_default();
    serde_json::from_str(&raw).unwrap_or_default()
}

fn save_sr_subscribed(user_id: u32, data: Vec<UserSubs>) {
    let mut conn = get_redis_conn();  // TODO: use connection pool
    let key = format!("user-{}-subs", user_id);

    let _ : () = conn.set(key, serde_json::to_string(&data).unwrap()).unwrap();
}

pub fn srlist_get(info: web::Path<(u32,)>) -> Result<HttpResponse> {
    let srlist = fetch_sr_subscribed(info.0);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(format!("{}", serde_json::to_string(&srlist)?))
    )
}

pub fn srlist_post(info: web::Path<(u32,)>) -> Result<HttpResponse> {
    let _subs = fetch_sr_subscribed(info.0);

     Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body("[]")
    )
}

pub fn pics_get(_info: web::Path<(u32,)>) -> Result<HttpResponse> {
    let mut conn = get_redis_conn();  // TODO: use connection pool
    let result: Vec<String> = conn.lrange("p1", 0, -1).unwrap();
    let pics: Vec<UserPic> = result.iter().map(|x| UserPic { url: x.clone() }).collect();

     Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(format!("{}", serde_json::to_string(&pics)?))
    )
}

pub fn pics_post(_info: web::Path<(u32,)>) -> Result<HttpResponse> {
    // (msg: Json<UserPic>) -> JsonValue {
    // let conn = get_redis_conn();  // TODO: use connection pool
    // let _: usize = conn.rpush("p1", &msg.url).unwrap();

     Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body("[]")
    )
}
