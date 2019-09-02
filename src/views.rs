//
// Each function is a view/controller for a HTTP route,
// takes a HttpRequest and returns a HttpResponse.
//

extern crate reqwest;
extern crate serde_derive;

use actix_web;
use futures::future::Future;
use reqwest::Url;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

use crate::models;
use crate::redditapi;
use crate::jwt;


pub fn testing(_req: actix_web::HttpRequest) -> impl Future<Item = actix_web::HttpResponse, Error = actix_web::Error> {
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

// Route: /
pub fn home(_req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    println!(">>> New home request.");
    let mut file = File::open("../frontend/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(contents))
}

// Route: /redditauth.html
// Redirect the client to the Reddit oAuth autorization page.
pub fn redditauth(_req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    println!(">>> New redditauth request, redirecting...");
    let state = Uuid::new_v4().to_string();
    let url = redditapi::get_reddit_authorize_url(state);
    Ok(actix_web::HttpResponse::Found().header(actix_web::http::header::LOCATION, url).finish())
}

// Route: /redditcallback.html
// After Reddit auth page: check state and use code to get first token.
pub fn redditcallback(params: actix_web::web::Query<redditapi::RedditAuthCallback>) -> actix_web::Result<actix_web::HttpResponse>
{
    println!(">>> New redditcallback request: code {:?} / state {:?}", &params.code, &params.state);
    let reddit_token = match redditapi::RedditAccessToken::new(&params.code) {
        Some(x) => x,
        None => return Ok(actix_web::HttpResponse::Ok().content_type("text/html").body("Invalid Token.")),
    };
    let reddit_user = redditapi::UserBasics::fetch(&reddit_token).unwrap_or_default();
    println!(">>> redditcallback UserBasics: {:?}", &reddit_user);

    let user = models::User::from_reddit(&reddit_user).unwrap_or_default();
    let jwt_token = jwt::JwtTokenToken::new(&user, &reddit_token);
    // TODO: Set a shorter expire time for the first JWT
    let url = format!("/home#x={}", &jwt_token.to_string().unwrap());
    Ok(actix_web::HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, url)
        .finish())
}

// Route: /home
// This route simply serves the static SPA with no further auth checks.
// Auth checks will be done later be the API endpoints.
pub fn app() -> actix_web::Result<actix_web::HttpResponse> {
    let mut file = File::open("../frontend/app.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(contents))
}
