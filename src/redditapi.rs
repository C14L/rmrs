/// Communicate with Reddit API endpoints.
extern crate reqwest;

use reqwest::Url;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use reqwest::header::USER_AGENT;
use std::error::Error;

use crate::helpers;

const REDDIT_TOKEN_URL: &'static str = "https://www.reddit.com/api/v1/access_token";
const APP_OAUTH_CB: &'static str = "http://localhost:8001/redditcallback.html";
const APP_NAME: &'static str = "BtNjVhBUlLJDXg";
const APP_SECRET: &'static str = "i5x4WPmHUA6Q7rYHB1SuOMemgSs";
const APP_USER_AGENT: &'static str = "web:reddmeet.com:v2.0.0a/rust (by /u/C14L)";

pub fn get_reddit_authorize_url(state: String) -> String {
    format!("https://ssl.reddit.com/api/v1/authorize?duration=permanent&response_type=code&\
        scope=identity,mysubreddits&redirect_uri={}&state={}&client_id={}",
        APP_OAUTH_CB, state, APP_NAME)
}

/// Redit Auth

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAuthCallback {
    pub state: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAccessToken {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<usize>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub update_time: Option<u64>,
    pub expire_time: Option<u64>,
    pub create_time: Option<u64>,
}

impl RedditAccessToken {
    pub fn new(code: &String) -> Option<Self> {
        println!(">>> RedditAccessToken::new() with code: {:?}", code);

        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", APP_OAUTH_CB),
        ];

        println!(">>> RedditAccessToken::new() --> body: {:?}", &body);
        println!(">>> RedditAccessToken --> url: {:?}", &url);

        let req = reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body);

        println!(">>> RedditAccessToken --> req: {:?}", &req);

        let mut res = req.send().unwrap();  // TODO: catch possible error!!!

        println!(">>> RedditAccessToken --> res: {:?}", &res);

        let json = res.json();

        println!(">>> RedditAccessToken --> json: {:?}", &json);

        json.and_then(|mut x: RedditAccessToken| {
            println!(">>> RedditAccessToken --> unmarshalled response: {:?}", &x);
            let t = helpers::unix_timestamp().expect("No time?");
            x.create_time = Some(t);
            x.update_time = Some(t);
            x.expire_time = Some(x.expires_in.unwrap() as u64 + t);
            Ok(x)
        })
        .or_else(|e| {
            println!(">>> RedditAccessToken --> ERROR unmarshalling response: {:?}", &e);
            Err(e)
        })
        .ok()
    }

    pub fn refresh(&mut self) -> Option<&Self> {
        let url = Url::parse(REDDIT_TOKEN_URL).unwrap();
        let body = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token.to_owned().unwrap()),
        ];

        let req = reqwest::Client::new()
            .post(url)
            .basic_auth(APP_NAME, Some(APP_SECRET))
            .header(USER_AGENT, APP_USER_AGENT)
            .form(&body);
        println!(">>> RedditAccessToken --> req: {:?}", &req);
        let mut res = req.send().unwrap();  // TODO: catch possible error!!!
        println!(">>> RedditAccessToken --> res: {:?}", &res);
        let json: Result<RedditAccessToken, _> = res.json();

        match json {
            Ok(x) => {
                println!(">>> RedditAccessToken --> json: {:?}", &x);
                let t = helpers::unix_timestamp().expect("No time?");
                self.expires_in = x.expires_in;
                self.refresh_token = x.refresh_token;
                self.update_time = Some(t);
                self.expire_time = Some(x.expires_in.unwrap() as u64 + t);
                Some(self)
            },
            Err(_) => None,
        }
    }
}

/// Basic User Data

#[derive(Default, Debug, Deserialize)]
pub struct UserBasics {
    name: String,
    created_utc: String,
    link_karma: i32,
    comment_karma: i32,
    over_18: bool,
    hide_from_robots: bool,
    has_verified_email: bool,
    gold_creddits: bool,
}

pub fn fetch_user_basics(_reddit_token: &RedditAccessToken) -> Result<UserBasics, reqwest::Error> {
    // TODO: with the reddit_token, fetch basic user data (username, etc) from reddit to put in JWT.
    Ok(UserBasics { name: "C14L".to_string(), ..Default::default() })
}

/// Trophies

#[derive(Debug, Deserialize)]
pub struct UserTrophyListTrophiesItemData {
    icon_70: Option<String>,
    name: Option<String>,
    url: Option<String>,
    icon_40: Option<String>,
    award_id: Option<String>,
    id: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserTrophyListTrophiesItem {
    kind: String,
    data: UserTrophyListTrophiesItemData,
}

#[derive(Debug, Deserialize)]
pub struct UserTrophyListTrophies {
    trophies: Vec<UserTrophyListTrophiesItem>,
}

#[derive(Debug, Deserialize)]
pub struct UserTrophyList {
    kind: String,
    data: UserTrophyListTrophies,
}

/// Comments

#[derive(Debug, Deserialize)]
pub struct UserCommentKindT1 {
    subreddit_id: Option<String>,
    approved_at_utc: Option<f64>,
    edited: Option<bool>,
    mod_reason_by: Option<String>,
    banned_by: Option<String>,
    author_flair_type: Option<String>,
    removal_reason: Option<String>,
    link_id: Option<String>,
    author_flair_template_id: Option<String>,
    likes: Option<bool>,
    replies: Option<String>,
    user_reports: Vec<String>,
    saved: Option<bool>,
    id: Option<String>,
    banned_at_utc: Option<f64>,
    mod_reason_title: Option<String>,
    gilded: Option<usize>,
    archived: Option<bool>,
    no_follow: Option<bool>,
    author: Option<String>,
    num_comments: Option<usize>,
    can_mod_post: Option<bool>,
    send_replies: Option<bool>,
    parent_id: Option<String>,
    score: Option<usize>,
    author_fullname: Option<String>,
    over_18: Option<bool>,
    approved_by: Option<String>,
    mod_note: Option<String>,
    controversiality: Option<usize>,
    body: Option<String>,
    link_title: Option<String>,
    downs: Option<usize>,
    author_flair_css_class: Option<String>,
    name: Option<String>,
    author_patreon_flair: Option<bool>,
    collapsed: Option<bool>,
    author_flair_richtext: Vec<String>,
    is_submitter: Option<bool>,
    body_html: Option<String>,
    gildings: HashMap<String, usize>,
    collapsed_reason: Option<String>,
    stickied: Option<bool>,
    can_gild: Option<bool>,
    subreddit: Option<String>,
    author_flair_text_color: Option<String>,
    score_hidden: Option<bool>,
    permalink: Option<String>,
    num_reports: Option<String>,
    link_permalink: Option<String>,
    report_reasons: Option<String>,
    link_author: Option<String>,
    author_flair_text: Option<String>,
    link_url: Option<String>,
    created: Option<f64>,
    created_utc: Option<f64>,
    subreddit_name_prefixed: Option<String>,
    distinguished: Option<String>,
    author_flair_background_color: Option<String>,
    rte_mode: Option<String>,
    mod_reports: Vec<String>,
    quarantine: Option<bool>,
    subreddit_type: Option<String>,
    ups: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct UserCommentT1 {
    kind: String,
    data: UserCommentKindT1,
}

#[derive(Debug, Deserialize)]
pub struct UserCommentData {
    modhash: String,
    dist: usize,
    children: Vec<UserCommentT1>,
}

#[derive(Debug, Deserialize)]
pub struct UserComments {
    kind: String,
    data: UserCommentData,
}

pub fn fetch_user_trophies(username: &str) -> Result<UserTrophyList, reqwest::Error> {
    let s = format!(
        "https://www.reddit.com/user/{}/trophies/.json?t=all",
        username
    );
    let url = Url::parse(&s).unwrap();
    let response: UserTrophyList = reqwest::get(url)?.json()?;
    Ok(response)
}

pub fn fetch_user_comments(username: &str) -> Result<UserComments, reqwest::Error> {
    let s = format!(
        "https://www.reddit.com/user/{}/comments/.json?t=all",
        username
    );
    let url = Url::parse(&s).unwrap();
    let response: UserComments = reqwest::get(url)?.json()?;
    Ok(response)
}
