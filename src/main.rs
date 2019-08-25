//#[macro_use] extern crate actix_web;

//
// Basic example:
//
// https://github.com/actix/examples/blob/master/basics/src/main.rs
//

use std::{env, io};

use actix_cors::Cors;
use actix_files as fs;
// use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http, middleware, web, App, HttpResponse, HttpServer};

mod api;
mod helpers;
mod models;
mod views;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    // env_logger::init();

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
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&[0; 32]).name("auth").secure(false))
            // )
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(views::home)))
            .service(web::resource("/testing").route(web::get().to_async(views::testing)))
            // Simply redirects the client to Reddit's oAuth page.
            .service(web::resource("/redditauth.html").route(web::get().to(views::redditauth)))
            // Redirected to by Reddit after auth okay.
            .service(
                web::resource("/redditcallback.html").route(web::get().to(views::redditcallback)),
            )
            // Load main app
            .service(web::resource("/home").route(web::get().to(views::app)))
            .service(web::resource("/api/{id}/srlist.json").route(web::get().to(api::srlist_get)))
            .service(web::resource("/api/{id}/srlist.json").route(web::post().to(api::srlist_post)))
            .service(web::resource("/api/{id}/pics.json").route(web::get().to(api::pics_get)))
            .service(web::resource("/api/{id}/pics.json").route(web::post().to(api::pics_post)))
            .service(fs::Files::new("/", "../frontend/").index_file("main.css"))
            .service(fs::Files::new("/", "../frontend/").index_file("main.js"))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8001")?
    .run()
}
