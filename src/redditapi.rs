// #![allow(dead_code)]

/// Communicate with Reddit API endpoints.

extern crate reqwest;

use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;

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
