/// Authenticate to access Reddit API endpoints.
/// https://github.com/reddit-archive/reddit/wiki/OAuth2

extern crate reqwest;

use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Url;
use serde::Deserialize;

const REDDIT_API_ENDPOINT: &'static str = "https://www.reddit.com/api/v1/access_token";
const API_ENDPOINT: &'static str = "http://localhost:8001/redditcallback.html";
const APP_NAME: &'static str = "BtNjVhBUlLJDXg";
const APP_SECRET: &'static str = "i5x4WPmHUA6Q7rYHB1SuOMemgSs";

#[database("redis_db")]
pub struct RedisDbConn(redis::Connection);

#[derive(FromForm)]
pub struct RedditCallbackReply {
    error: Option<String>,
    code: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RedditAccessToken {
    init_state: Option<String>, // to verify when obtaining initial code
    init_code: Option<String>,  // to fetch the initial access token
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<usize>,
    refresh_token: Option<String>,
    scope: Option<String>,
    update_time: Option<u64>,
    expire_time: Option<u64>,
    create_time: Option<u64>,
}

impl RedditAccessToken {

    pub fn new(code: String) -> Result<RedditAccessToken, reqwest::Error> {
        println!("THIS IS RedditAccessToken::new()");
        let body = [
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", "http://localhost:8001/redditcallback.html"),
        ];
        reqwest::Client::new()
            .post(Url::parse(REDDIT_API_ENDPOINT).unwrap())
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .form(&body)
            .send()?
            .json()
            .and_then(|mut x: RedditAccessToken| {
                let t = SystemTime::now().duration_since(UNIX_EPOCH).expect("No time?").as_secs();
                x.create_time = Some(t);
                x.update_time = Some(t);
                x.expire_time = x.expires_in.map(|x| x as u64 + t);
                Ok(x)
            })
    }

    pub fn refresh(self) -> Result<RedditAccessToken, reqwest::Error> {
        let body = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token.expect("No token!").as_str()),
        ];
        reqwest::Client::new()
            .post(Url::parse(REDDIT_API_ENDPOINT).unwrap())
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .form(&body)
            .send()?
            .json()
            .and_then(|mut x: RedditAccessToken| {
                let t = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("No time?")
                    .as_secs();
                x.update_time = Some(t);
                x.expire_time = x.expires_in.map(|x| x as u64 + t);
                Ok(x)
            })
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthSession {
    sess_id: String,
    token: Option<RedditAccessToken>,
}

impl AuthSession {
    pub fn new(sess_id: &str) -> AuthSession {
        let redis_key = format!("sess-{}", sess_id);

        AuthSession { sess_id: sess_id.into(), token: None }
    }
    pub fn set_token(mut self, token: RedditAccessToken) -> AuthSession {
        self.token = Some(token);
        self
    }
}


// Simply redirects the client to Reddit's oAuth page.
#[get("/redditauth.html")]
pub fn oauth_call_get(
    redis_conn: RedisDbConn,
    mut cookies: Cookies,
) -> Result<Redirect, NotFound<String>> {
    // TODO: Generate a `state` uuid and remember it to verify in oauth_callback.
    // TODO: Load `client_id` and `redirect_uri` from some conf file.

    let _sess_cookie = match cookies.get_private("rmrs_sessid") {
        Some(cookie) => {
            // There is a session cookie, check if it has a valid token in Redis.
            let redis_key = format!("sess-{}", cookie.value());
            let sess: Result<RedditAccessToken, _> = redis_conn.hgetall(redis_key);

            if sess.is_some() {
                // If it does, try to refresh it.
                println!("FOUND A REFRESH TOKEN")
                // If it refreshes, auth was sucessful --> redir to app.

            }

            // If any of that fails, re-use cookie and we need to go through oauth flow.
            cookie
        },
        None => {
            // No sess cookie found, create one, then go through oauth flow
            let random_uuid = "a8yr7ersdfsd4iuyhli54-=09u8hfi3;fdk-0";
            let cookie = Cookie::new("rmrs_sessid", random_uuid);
            cookies.add_private(cookie.clone());
            let redis_key = format!("sess-{}", cookie.value());
            let _: Option<String> = redis_conn.hset(redis_key, "refresh_token", "").ok();
            cookie
        }
    };

    Ok(Redirect::to(format!(
        "https://ssl.reddit.com/api/v1/authorize?\
         duration=permanent&response_type=code&scope=identity&\
         redirect_uri={}&state=kfhldhdgdfggeruhnstringlgp09u40hor&\
         client_id={}", API_ENDPOINT, APP_NAME
    )))
}

#[get("/redditcallback.html?<params..>")]
pub fn oauth_callback_get(
    mut cookies: Cookies,
    params: Form<RedditCallbackReply>,
) -> Result<Html<String>, NotFound<String>> {
    println!("THIS IS: oauth_callback_get()");

    let sess_cookie = cookies.get_private("rmrs_sessid").unwrap_or_else(|| {
        // This is weird, because a session should have been created before
        // before the oauth redirect. But for now, just go with it.
        let random_uuid = "a8yr7ersdfsd4iuyhli54-=09u8hfi3;fdk-0";
        let cookie = Cookie::new("rmrs_sessid", random_uuid);
        cookies.add_private(cookie.clone());
        cookie
    });
    println!("COOKIE: {:?}", &sess_cookie);

    let reddit_token: Option<RedditAccessToken> = params.code.clone().and_then(
        |c| RedditAccessToken::new(c).ok());

    let sess: Option<AuthSession> = reddit_token.and_then(
        |t| Some(AuthSession::new(sess_cookie.value()).set_token(t)));

    println!("SESS: {:?}", &sess);

    match sess {
        Some(s) => {
            let token = s.token.unwrap();
            Ok(Html(format!(
                "session_id: '{}' and state: '{}' -- access_token: '{}', token_type: '{}', expires_in: '{}', scope: '{}', refresh_token: '{}'.",
                sess_cookie.value(),
                params.state.clone().unwrap_or_default(),
                token.access_token,
                token.token_type,
                token.expires_in,
                token.scope,
                token.refresh_token,
            )))
        },
        None => Err(NotFound("Invalid token.".to_string()))
    }
}
