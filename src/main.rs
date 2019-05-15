#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::Method;
use rocket::routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use std::collections::HashMap;
use std::ffi::OsString;

mod api;
mod pages;
mod redditapi;
mod redditauth;


fn main() -> Result<(), Error> {

    // println!("Fetching comments from Reddit...");
    // let comments = redditapi::fetch_user_comments("c14l");
    // println!("{:?}", comments);

    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:8000", "http://localhost:8001"]);
    let mut html_pages: HashMap<OsString, String> = HashMap::new();
    pages::preload_static_pages(&mut html_pages);

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
        .mount("/api/v2", routes![
            api::srlist_get, api::srlist_post, api::pics_get, api::pics_post])
        .mount("/", routes![pages::home, pages::settings, redditauth::oauth_callback_get])
        .attach(cors)
        .manage(html_pages)
        .launch();

    Ok(())
}
