//#[macro_use] extern crate actix_web;

//
// Basic example:
//
// https://github.com/actix/examples/blob/master/basics/src/main.rs
//

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, middleware, web, App, HttpResponse, HttpServer};
use env_logger;
use std::env;

pub mod api;
pub mod conf;
pub mod helpers;
pub mod jwt;
pub mod models;
pub mod redditapi;
pub mod views;

fn main() { // -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://127.0.0.1:8001/")
                    .allowed_methods(vec!["HEAD", "GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "rmrs-0.0.1"))
            .wrap(middleware::Logger::default())
            //.wrap(middleware::Logger::new("%a %{User-Agent}i"))
            // Define routes
            .service(web::resource("/").route(web::get().to(views::home)))
            .service(web::resource("/testing").route(web::get().to_async(views::testing)))
            // View routes for oAuth flow
            .service(web::resource("/redditauth.html").route(web::get().to(views::redditauth)))
            .service(web::resource("/redditcallback.html").route(web::get().to(views::redditcallback)))
            .service(web::resource("/home").route(web::get().to(views::app)))
            // API routes
            .service(web::resource("/api/user/me.json").route(web::get().to(api::user_me_get)))
            .service(web::resource("/api/user/me.json").route(web::get().to(api::user_me_post)))
            .service(web::resource("/api/user/me/subs.json").route(web::post().to(api::user_me_subs_post)))
            .service(web::resource("/api/user/me/pics.json").route(web::post().to(api::user_me_pics_post)))
            // Static file routes
            .service(fs::Files::new("/", "../frontend/").index_file("main.css"))
            .service(fs::Files::new("/", "../frontend/").index_file("main.js"))
            // Errors
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8001").unwrap().run().unwrap();
}
