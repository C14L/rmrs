// Helper functions
//

// use std::borrow::Cow;
// use actix_web::cookie::Cookie;
use actix_web::HttpRequest;

pub struct User {
    pub id: u64,
    pub username: String,
}

impl User {
    pub fn from(_req: &HttpRequest) -> Option<User> {
        Some(User {
            id: 123,
            username: String::from("abc123"),
        })
    }
}

// pub fn get_sessid(req: &HttpRequest) -> Option<String> {
    // req.headers().get("cookie")
        // .map(|x| x.to_str().ok().and_then(|x| Some(Cow::from(x))))
        // .unwrap_or(None)
        // .map(|x| Cookie::parse(x).ok().and_then(|x| Some(x.value().to_string())))
        // .unwrap_or(None)
// }
