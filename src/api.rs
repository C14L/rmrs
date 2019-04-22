/// Endpoints accessible on the /api route.

extern crate redis;

use redis::Commands;
use rocket::get;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPic {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SrState {
    sr: String,
    state: String,
}

#[get("/pics.json", format = "json", rank = 1)]
pub fn pics_get() -> JsonValue {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let con = client.get_connection().unwrap();
    let result: Vec<String> = con.lrange("p1", 0, -1).unwrap();
    let pics: Vec<UserPic> = result.iter().map(|x| UserPic { url: x.clone() }).collect();
    json!(pics)
}

#[post("/pics.json", format = "json", data = "<msg>", rank = 3)]
pub fn pics_post(msg: Json<UserPic>) -> JsonValue {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let con = client.get_connection().unwrap();
    let _: usize = con.rpush("p1", &msg.url).unwrap();
    json!({ "status": "ok" })  // TODO: Respond here with a simple HTTP 200
}

#[get("/srlist.json", format = "json", rank = 1)]
pub fn srlist_get() -> JsonValue {
    let srlist = vec![
        SrState {
            sr: String::from("Adsense"),
            state: String::from("1"),
        },
        SrState {
            sr: String::from("analytics"),
            state: String::from("1"),
        },
        SrState {
            sr: String::from("Android"),
            state: String::from("1"),
        },
    ];
    json!(srlist)
}

#[post("/srlist.json", format = "json", data = "<msg>", rank = 2)]
pub fn srlist_post(msg: Json<SrState>) -> JsonValue {
    println!("{:?}", &msg);
    json!({ "status": "ok" })  // TODO: Respond here with a simple HTTP 200
}
