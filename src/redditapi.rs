#![allow(dead_code)]

/// Communicate with Reddit API endpoints.

extern crate reqwest;

use reqwest::Url;
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use reqwest::header::USER_AGENT;

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
        }).ok()
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

// #[derive(Default, Debug, Deserialize)]
// pub struct UserBasics {
//     name: String,
//     created_utc: f32,
//     link_karma: i32,
//     comment_karma: i32,
//     over_18: bool,
//     hide_from_robots: bool,
//     has_verified_email: bool,
//     gold_creddits: bool,
// }

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserBasics {
    #[serde(rename = "is_employee")]
    pub is_employee: bool,

    #[serde(rename = "seen_layout_switch")]
    pub seen_layout_switch: bool,

    #[serde(rename = "has_visited_new_profile")]
    pub has_visited_new_profile: bool,

    #[serde(rename = "pref_no_profanity")]
    pub pref_no_profanity: bool,

    #[serde(rename = "has_external_account")]
    pub has_external_account: bool,

    #[serde(rename = "pref_geopopular")]
    pub pref_geopopular: String,

    #[serde(rename = "seen_redesign_modal")]
    pub seen_redesign_modal: bool,

    #[serde(rename = "pref_show_trending")]
    pub pref_show_trending: bool,

    #[serde(rename = "subreddit")]
    pub subreddit: Subreddit,

    #[serde(rename = "is_sponsor")]
    pub is_sponsor: bool,

    #[serde(rename = "gold_expiration")]
    pub gold_expiration: Option<serde_json::Value>,

    #[serde(rename = "has_gold_subscription")]
    pub has_gold_subscription: bool,

    #[serde(rename = "num_friends")]
    pub num_friends: i64,

    #[serde(rename = "features")]
    pub features: Features,

    #[serde(rename = "has_android_subscription")]
    pub has_android_subscription: bool,

    #[serde(rename = "verified")]
    pub verified: bool,

    #[serde(rename = "pref_autoplay")]
    pub pref_autoplay: bool,

    #[serde(rename = "coins")]
    pub coins: i64,

    #[serde(rename = "has_paypal_subscription")]
    pub has_paypal_subscription: bool,

    #[serde(rename = "has_subscribed_to_premium")]
    pub has_subscribed_to_premium: bool,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "has_stripe_subscription")]
    pub has_stripe_subscription: bool,

    #[serde(rename = "seen_premium_adblock_modal")]
    pub seen_premium_adblock_modal: bool,

    #[serde(rename = "can_create_subreddit")]
    pub can_create_subreddit: bool,

    #[serde(rename = "over_18")]
    pub over_18: bool,

    #[serde(rename = "is_gold")]
    pub is_gold: bool,

    #[serde(rename = "is_mod")]
    pub is_mod: bool,

    #[serde(rename = "suspension_expiration_utc")]
    pub suspension_expiration_utc: Option<serde_json::Value>,

    #[serde(rename = "has_verified_email")]
    pub has_verified_email: bool,

    #[serde(rename = "is_suspended")]
    pub is_suspended: bool,

    #[serde(rename = "pref_video_autoplay")]
    pub pref_video_autoplay: bool,

    #[serde(rename = "in_redesign_beta")]
    pub in_redesign_beta: bool,

    #[serde(rename = "icon_img")]
    pub icon_img: String,

    #[serde(rename = "pref_nightmode")]
    pub pref_nightmode: bool,

    #[serde(rename = "oauth_client_id")]
    pub oauth_client_id: String,

    #[serde(rename = "hide_from_robots")]
    pub hide_from_robots: bool,

    #[serde(rename = "link_karma")]
    pub link_karma: i64,

    #[serde(rename = "force_password_reset")]
    pub force_password_reset: bool,

    #[serde(rename = "inbox_count")]
    pub inbox_count: i64,

    #[serde(rename = "pref_top_karma_subreddits")]
    pub pref_top_karma_subreddits: bool,

    #[serde(rename = "pref_show_snoovatar")]
    pub pref_show_snoovatar: bool,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "pref_clickgadget")]
    pub pref_clickgadget: i64,

    #[serde(rename = "created")]
    pub created: f64,

    #[serde(rename = "gold_creddits")]
    pub gold_creddits: i64,

    #[serde(rename = "created_utc")]
    pub created_utc: f64,

    #[serde(rename = "has_ios_subscription")]
    pub has_ios_subscription: bool,

    #[serde(rename = "pref_show_twitter")]
    pub pref_show_twitter: bool,

    #[serde(rename = "in_beta")]
    pub in_beta: bool,

    #[serde(rename = "comment_karma")]
    pub comment_karma: i64,

    #[serde(rename = "has_subscribed")]
    pub has_subscribed: bool,

    #[serde(rename = "seen_subreddit_chat_ftux")]
    pub seen_subreddit_chat_ftux: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "chat_subreddit")]
    pub chat_subreddit: bool,

    #[serde(rename = "show_amp_link")]
    pub show_amp_link: bool,

    #[serde(rename = "read_from_pref_service")]
    pub read_from_pref_service: bool,

    #[serde(rename = "chat_rollout")]
    pub chat_rollout: bool,

    #[serde(rename = "chat")]
    pub chat: bool,

    #[serde(rename = "chat_reddar_reports")]
    pub chat_reddar_reports: bool,

    #[serde(rename = "default_srs_holdout")]
    pub default_srs_holdout: DefaultSrsHoldout,

    #[serde(rename = "twitter_embed")]
    pub twitter_embed: bool,

    #[serde(rename = "is_email_permission_required")]
    pub is_email_permission_required: bool,

    #[serde(rename = "richtext_previews")]
    pub richtext_previews: bool,

    #[serde(rename = "chat_subreddit_notification_ftux")]
    pub chat_subreddit_notification_ftux: bool,

    #[serde(rename = "mod_awards")]
    pub mod_awards: bool,

    #[serde(rename = "mweb_xpromo_revamp_v3")]
    pub mweb_xpromo_revamp_v3: DefaultSrsHoldout,

    #[serde(rename = "mweb_xpromo_revamp_v2")]
    pub mweb_xpromo_revamp_v2: DefaultSrsHoldout,

    #[serde(rename = "mweb_xpromo_modal_listing_click_daily_dismissible_ios")]
    pub mweb_xpromo_modal_listing_click_daily_dismissible_ios: bool,

    #[serde(rename = "community_awards")]
    pub community_awards: bool,

    #[serde(rename = "modlog_copyright_removal")]
    pub modlog_copyright_removal: bool,

    #[serde(rename = "dual_write_user_prefs")]
    pub dual_write_user_prefs: bool,

    #[serde(rename = "do_not_track")]
    pub do_not_track: bool,

    #[serde(rename = "chat_user_settings")]
    pub chat_user_settings: bool,

    #[serde(rename = "mweb_xpromo_interstitial_comments_ios")]
    pub mweb_xpromo_interstitial_comments_ios: bool,

    #[serde(rename = "mweb_xpromo_modal_listing_click_daily_dismissible_android")]
    pub mweb_xpromo_modal_listing_click_daily_dismissible_android: bool,

    #[serde(rename = "premium_subscriptions_table")]
    pub premium_subscriptions_table: bool,

    #[serde(rename = "mweb_xpromo_interstitial_comments_android")]
    pub mweb_xpromo_interstitial_comments_android: bool,

    #[serde(rename = "chat_group_rollout")]
    pub chat_group_rollout: bool,

    #[serde(rename = "custom_feeds")]
    pub custom_feeds: bool,

    #[serde(rename = "spez_modal")]
    pub spez_modal: bool,

    #[serde(rename = "mweb_link_tab")]
    pub mweb_link_tab: DefaultSrsHoldout,

    #[serde(rename = "layers_creation")]
    pub layers_creation: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DefaultSrsHoldout {
    #[serde(rename = "owner")]
    pub owner: String,

    #[serde(rename = "variant")]
    pub variant: String,

    #[serde(rename = "experiment_id")]
    pub experiment_id: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Subreddit {
    #[serde(rename = "default_set")]
    pub default_set: bool,

    #[serde(rename = "user_is_contributor")]
    pub user_is_contributor: bool,

    #[serde(rename = "banner_img")]
    pub banner_img: String,

    #[serde(rename = "restrict_posting")]
    pub restrict_posting: bool,

    #[serde(rename = "user_is_banned")]
    pub user_is_banned: bool,

    #[serde(rename = "free_form_reports")]
    pub free_form_reports: bool,

    #[serde(rename = "community_icon")]
    pub community_icon: String,

    #[serde(rename = "show_media")]
    pub show_media: bool,

    #[serde(rename = "icon_color")]
    pub icon_color: String,

    #[serde(rename = "user_is_muted")]
    pub user_is_muted: bool,

    #[serde(rename = "display_name")]
    pub display_name: String,

    #[serde(rename = "header_img")]
    pub header_img: Option<serde_json::Value>,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "coins")]
    pub coins: i64,

    #[serde(rename = "over_18")]
    pub over_18: bool,

    #[serde(rename = "icon_size")]
    pub icon_size: Vec<i64>,

    #[serde(rename = "primary_color")]
    pub primary_color: String,

    #[serde(rename = "icon_img")]
    pub icon_img: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "submit_link_label")]
    pub submit_link_label: String,

    #[serde(rename = "header_size")]
    pub header_size: Option<serde_json::Value>,

    #[serde(rename = "restrict_commenting")]
    pub restrict_commenting: bool,

    #[serde(rename = "subscribers")]
    pub subscribers: i64,

    #[serde(rename = "submit_text_label")]
    pub submit_text_label: String,

    #[serde(rename = "is_default_icon")]
    pub is_default_icon: bool,

    #[serde(rename = "link_flair_position")]
    pub link_flair_position: String,

    #[serde(rename = "display_name_prefixed")]
    pub display_name_prefixed: String,

    #[serde(rename = "key_color")]
    pub key_color: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "is_default_banner")]
    pub is_default_banner: bool,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "banner_size")]
    pub banner_size: Option<serde_json::Value>,

    #[serde(rename = "user_is_moderator")]
    pub user_is_moderator: bool,

    #[serde(rename = "public_description")]
    pub public_description: String,

    #[serde(rename = "link_flair_enabled")]
    pub link_flair_enabled: bool,

    #[serde(rename = "disable_contributor_requests")]
    pub disable_contributor_requests: bool,

    #[serde(rename = "subreddit_type")]
    pub subreddit_type: String,

    #[serde(rename = "user_is_subscriber")]
    pub user_is_subscriber: bool,
}

impl UserBasics {
    pub fn fetch(reddit_token: &RedditAccessToken) -> Result<Self, reqwest::Error> {
        println!(">>> UserBasics::fetch");
        let url = Url::parse("https://oauth.reddit.com/api/v1/me.json").unwrap();
        let req = reqwest::Client::new()
            .get(url)
            .query(&[("raw_json", "1")])
            .header("Authorization", format!("bearer {}", reddit_token.access_token.as_ref().unwrap()))
            .header(USER_AGENT, APP_USER_AGENT);
        println!(">>> UserBasics::fetch --> req: {:?}", &req);
        let mut res = req.send()?;  // TODO: catch possible reqwest error and convert into()
        // println!(">>> UserBasics::fetch --> res: {:?}", &res);
        // println!(">>> UserBasics::fetch --> res TEXT: {:?}", &res.text());

        res.json::<Self>()
        // .and_then(|x: Self| {
        //     println!(">>> JSON UserBasics::fetch --> res: {:?}", &x);
        //     Ok(Self { name: x.name.to_owned(), ..Default::default() })
        // })
        .or_else(|e| {
            println!(">>> + + + ERROR + + + UserBasics::fetch --> {:?}", e);
            Ok(Self { ..Default::default() })
        })
    }
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
