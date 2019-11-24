//
// Each function is a view/controller for a HTTP route,
// takes a HttpRequest and returns a HttpResponse.
//

extern crate reqwest;
extern crate serde_derive;

use actix_web::http::header::LOCATION;
use actix_web::Error as ActixError;
use actix_web::Result as ActixResult;
use actix_web::{web, HttpResponse};
use futures::future::Future;
use reqwest::Url;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

use crate::jwt;
use crate::models::app_user::AppUser;
use crate::models::reddit_token::{RedditAuthCallback, RedditToken};
use crate::models::reddit_user::RedditUser;

const CONTENT_TYPE: &'static str = "text/html; charset=utf-8";

fn short_html(msg: String) -> ActixResult<HttpResponse> {
    let c = format!(r#"<!DOCTYPE html><html><head><meta charset="utf-8">
                    </head><body>{}</body></html>"#, msg);
    Ok(HttpResponse::Ok().content_type(CONTENT_TYPE).body(c))
}

pub fn testing() -> impl Future<Item = HttpResponse, Error = ActixError> {
    let url = Url::parse("https://example.com/").unwrap();
    let reqw = reqwest::Client::new().get(url);
    web::block(move || reqw.send())
        .from_err()
        .and_then(|_res| HttpResponse::Ok().content_type(CONTENT_TYPE).body("Hello!"))
}

// Route: /
pub fn home() -> ActixResult<HttpResponse> {
    let mut file = File::open("../frontend/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(HttpResponse::Ok().content_type(CONTENT_TYPE).body(contents))
}

// Route: /redditauth.html
// Redirect the client to the Reddit oAuth autorization page.
pub fn redditauth() -> ActixResult<HttpResponse> {
    let state = Uuid::new_v4().to_string();
    let url = RedditToken::get_auth_url(state);
    Ok(HttpResponse::Found().header(LOCATION, url).finish())
}

// Route: /redditcallback.html
// After Reddit auth page: check state and use code to get first token.
pub fn redditcallback(params: web::Query<RedditAuthCallback>) -> ActixResult<HttpResponse> {
    let mut reddit_token = match RedditToken::new(&params.code) {
        Ok(x) => x,
        Err(e) => return short_html(format!("Invalid Token (fn redditcallback): {:?}", e)),
    };
    let reddit_user = RedditUser::fetch_me(&mut reddit_token).unwrap_or_default();

    // Check if this user already has an account
    let user = match AppUser::load(&reddit_user.name) {
        Ok(app_user_loaded) => app_user_loaded,
        Err(_) => {
            match AppUser::from_reddit(&reddit_user) {
                Ok(app_user_from_reddit) => {
                    &app_user_from_reddit.save(); // Write to storage
                    app_user_from_reddit
                }
                Err(_) => return short_html("User not found.".into()),
            }
        }
    };

    let jwt_token = match jwt::JwtTokenToken::new(&user, &reddit_token) {
        Ok(res) => res,
        Err(_) => return short_html("Token create error.".into()),
    };

    let contents = format!(
        r#"<!DOCTYPE html>
        <html lang="en"><head><meta charset="UTF-8">
        <meta name="jwt" content="{}">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>Loading...</title></head><body>Loading...</body><script>
        let token = document.querySelector("meta[name='jwt']").getAttribute("content")
        localStorage.setItem('token', token);
        window.location.href = "/home";
        </script></html>"#,
        &jwt_token.to_string().unwrap()
    );
    Ok(HttpResponse::Ok().content_type(CONTENT_TYPE).body(contents))
}

// Route: /home
// This route simply serves the static SPA with no further auth checks.
// Auth checks will be done later be the API endpoints.
pub fn app() -> ActixResult<HttpResponse> {
    let mut file = File::open("../frontend/app.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(HttpResponse::Ok().content_type(CONTENT_TYPE).body(contents))
}
