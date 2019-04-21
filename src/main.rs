#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::Method;
use rocket::{get, routes};
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use serde::{Deserialize, Serialize};

mod pages;

#[derive(Serialize, Deserialize, Debug)]
struct UserPic {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SrState {
    sr: String,
    state: String,
}

#[get("/pics.json", format = "json", rank = 1)]
fn pics_get() -> JsonValue {
    let pics = vec![
        UserPic { url: String::from("https://i.imgur.com/jbaAGJHm.jpg") },
        UserPic { url: String::from("https://i.imgur.com/vPrly7Dm.jpg") },
        UserPic { url: String::from("https://i.imgur.com/HdIW12nm.jpg") },
        UserPic { url: String::from("https://i.imgur.com/GewGxsom.jpg") },
    ];
    json!(pics)
}

#[get("/srlist.json", format = "json", rank = 1)]
fn srlist_get() -> JsonValue {
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
fn srlist_post(msg: Json<SrState>) -> JsonValue {
    println!("{:?}", &msg);
    json!({ "status": "ok" })
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8000"]);
    let pages_filesnames = pages::fetch_pages_filenames();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Accept", "Content-Type"]),
        allow_credentials: false,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/api/v2", routes![srlist_get, srlist_post, pics_get])
        .mount("/", routes![pages::home])
        .attach(cors)
        .manage(pages_filesnames)
        .launch();

    Ok(())
}
