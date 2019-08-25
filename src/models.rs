// Data models for this app
//

use actix_web::HttpRequest;

#[derive(Default)]
pub struct User {
    pub id: u64,
    pub username: String,
}

impl User {
    pub fn from(_req: &HttpRequest) -> Option<Self> {
        None
        // Some(Self {
        //     id: 123,
        //     username: String::from("abc123"),
        // })
    }
}
