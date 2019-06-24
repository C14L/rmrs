//
// Each function is a view/controller for a HTTP route,
// takes a HttpRequest and returns a HttpResponse.
//

use actix_web::http::{header, StatusCode};
use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::middleware::identity::Identity;

use crate::helpers;
use crate::helpers::User;

pub fn home(req: HttpRequest) -> Result<HttpResponse> {
    match User::from(&req) {
        Some(u) => Ok(format!("Hello, you are {}!", u.username).into()),
        None => Ok(format!("Hey, you need <a href=/>to authenticate</a> first!").into()),
    }
}

// Make sure there is a session cookie and then
// redirect the client to Reddit's oAuth page.
pub fn redditauth(_req: HttpRequest, id: Identity) -> Result<HttpResponse> {
    id.remember("session-cookie".to_owned());
    // TODO: this should redirect to reddit oauth page
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!("Sesson Id created, now go to <a href='/redditcallback.html'>redditcallback</a>.")))
}

// Redirected to from Reddit after successful oauth.
// A session cookie must exist at this point.
pub fn redditcallback(req: HttpRequest) -> Result<HttpResponse> {
    // TODO: reddit oauth flow
    match helpers::get_sessid(&req) {
        Some(sessid) => {
            Ok(format!("Your session id: {:?}", &sessid).into())
        },
        None => {
            Ok(HttpResponse::Found()
                .header(header::LOCATION, "/redditauth.html")
                .finish())
        },
    }
}

//pub fn redditauth(req: HttpRequest, id: Identity, info: web::Path<(String, u32)>) -> Result<HttpResponse> {
