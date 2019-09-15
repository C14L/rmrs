extern crate serde_json;

use serde::{Serialize, Deserialize};

use crate::helpers::AppResult;
use crate::models::reddit_token::RedditToken;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RedditUserSubredditList {
    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "data")]
    data: RedditUserSubredditData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RedditUserSubredditData {
    #[serde(rename = "modhash")]
    modhash: Option<serde_json::Value>,

    #[serde(rename = "dist")]
    dist: i32,

    #[serde(rename = "children")]
    children: Vec<Child>,

    #[serde(rename = "after")]
    after: String,

    #[serde(rename = "before")]
    before: Option<serde_json::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Child {
    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "data")]
    data: ChildData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChildData {
    #[serde(rename = "user_is_banned")]
    user_is_banned: bool,

    #[serde(rename = "user_is_muted")]
    user_is_muted: bool,

    #[serde(rename = "display_name")]
    display_name: String,

    #[serde(rename = "header_img")]
    header_img: Option<String>,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "primary_color")]
    primary_color: String,

    #[serde(rename = "active_user_count")]
    active_user_count: Option<serde_json::Value>,

    #[serde(rename = "icon_img")]
    icon_img: String,

    #[serde(rename = "subscribers")]
    subscribers: i32,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "quarantine")]
    quarantine: bool,

    #[serde(rename = "advertiser_category")]
    advertiser_category: String,

    #[serde(rename = "user_has_favorited")]
    user_has_favorited: bool,

    #[serde(rename = "community_icon")]
    community_icon: String,

    #[serde(rename = "banner_background_image")]
    banner_background_image: String,

    #[serde(rename = "created")]
    created: f64,

    #[serde(rename = "user_is_subscriber")]
    user_is_subscriber: bool,

    #[serde(rename = "user_flair_text")]
    user_flair_text: Option<serde_json::Value>,

    #[serde(rename = "banner_background_color")]
    banner_background_color: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "user_is_moderator")]
    user_is_moderator: bool,

    #[serde(rename = "over18")]
    over18: bool,

    #[serde(rename = "lang")]
    lang: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "created_utc")]
    created_utc: f64,

    #[serde(rename = "user_is_contributor")]
    user_is_contributor: bool,

    // #[serde(rename = "public_description")]
    // public_description: String,

    // #[serde(rename = "description")]
    // description: String,

    // #[serde(rename = "user_flair_background_color")]
    // user_flair_background_color: Option<serde_json::Value>,

    // #[serde(rename = "submit_text_html")]
    // submit_text_html: Option<String>,

    // #[serde(rename = "restrict_posting")]
    // restrict_posting: bool,

    // #[serde(rename = "free_form_reports")]
    // free_form_reports: bool,

    // #[serde(rename = "wiki_enabled")]
    // wiki_enabled: bool,

    // #[serde(rename = "user_can_flair_in_sr")]
    // user_can_flair_in_sr: Option<serde_json::Value>,

    // #[serde(rename = "icon_size")]
    // icon_size: Option<serde_json::Value>,

    // #[serde(rename = "display_name_prefixed")]
    // display_name_prefixed: String,

    // #[serde(rename = "accounts_active")]
    // accounts_active: Option<serde_json::Value>,

    // #[serde(rename = "public_traffic")]
    // public_traffic: bool,

    // #[serde(rename = "user_flair_richtext")]
    // user_flair_richtext: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "videostream_links_count")]
    // videostream_links_count: i32,

    // #[serde(rename = "hide_ads")]
    // hide_ads: bool,

    // #[serde(rename = "emojis_enabled")]
    // emojis_enabled: bool,

    // #[serde(rename = "comment_score_hide_mins")]
    // comment_score_hide_mins: i32,

    // #[serde(rename = "user_flair_template_id")]
    // user_flair_template_id: Option<serde_json::Value>,

    // #[serde(rename = "original_content_tag_enabled")]
    // original_content_tag_enabled: bool,

    // #[serde(rename = "submit_text")]
    // submit_text: String,

    // #[serde(rename = "spoilers_enabled")]
    // spoilers_enabled: bool,

    // #[serde(rename = "header_title")]
    // header_title: Option<serde_json::Value>,

    // #[serde(rename = "header_size")]
    // header_size: Option<Vec<i32>>,

    // #[serde(rename = "user_flair_position")]
    // user_flair_position: String,

    // #[serde(rename = "all_original_content")]
    // all_original_content: bool,

    // #[serde(rename = "has_menu_widget")]
    // has_menu_widget: bool,

    // #[serde(rename = "is_enrolled_in_new_modmail")]
    // is_enrolled_in_new_modmail: Option<serde_json::Value>,

    // #[serde(rename = "key_color")]
    // key_color: String,

    // #[serde(rename = "can_assign_user_flair")]
    // can_assign_user_flair: bool,

    // #[serde(rename = "wls")]
    // wls: i32,

    // #[serde(rename = "show_media_preview")]
    // show_media_preview: bool,

    // #[serde(rename = "submission_type")]
    // submission_type: String,

    // #[serde(rename = "disable_contributor_requests")]
    // disable_contributor_requests: bool,

    // #[serde(rename = "allow_videogifs")]
    // allow_videogifs: bool,

    // #[serde(rename = "user_flair_type")]
    // user_flair_type: String,

    // #[serde(rename = "collapse_deleted_comments")]
    // collapse_deleted_comments: bool,

    // #[serde(rename = "emojis_custom_size")]
    // emojis_custom_size: Option<serde_json::Value>,

    // #[serde(rename = "public_description_html")]
    // public_description_html: String,

    // #[serde(rename = "allow_videos")]
    // allow_videos: bool,

    // #[serde(rename = "is_crosspostable_subreddit")]
    // is_crosspostable_subreddit: bool,

    // #[serde(rename = "notification_level")]
    // notification_level: String,

    // #[serde(rename = "can_assign_link_flair")]
    // can_assign_link_flair: bool,

    // #[serde(rename = "accounts_active_is_fuzzed")]
    // accounts_active_is_fuzzed: bool,

    // #[serde(rename = "submit_text_label")]
    // submit_text_label: Option<String>,

    // #[serde(rename = "link_flair_position")]
    // link_flair_position: String,

    // #[serde(rename = "user_sr_flair_enabled")]
    // user_sr_flair_enabled: Option<serde_json::Value>,

    // #[serde(rename = "user_flair_enabled_in_sr")]
    // user_flair_enabled_in_sr: bool,

    // #[serde(rename = "allow_discovery")]
    // allow_discovery: bool,

    // #[serde(rename = "user_sr_theme_enabled")]
    // user_sr_theme_enabled: bool,

    // #[serde(rename = "link_flair_enabled")]
    // link_flair_enabled: bool,

    // #[serde(rename = "subreddit_type")]
    // subreddit_type: String,

    // #[serde(rename = "suggested_comment_sort")]
    // suggested_comment_sort: Option<serde_json::Value>,

    // #[serde(rename = "banner_img")]
    // banner_img: String,

    // #[serde(rename = "show_media")]
    // show_media: bool,

    // #[serde(rename = "submit_link_label")]
    // submit_link_label: Option<String>,

    // #[serde(rename = "user_flair_text_color")]
    // user_flair_text_color: Option<serde_json::Value>,

    // #[serde(rename = "restrict_commenting")]
    // restrict_commenting: bool,

    // #[serde(rename = "user_flair_css_class")]
    // user_flair_css_class: Option<serde_json::Value>,

    // #[serde(rename = "allow_images")]
    // allow_images: bool,

    // #[serde(rename = "whitelist_status")]
    // whitelist_status: String,

    // #[serde(rename = "banner_size")]
    // banner_size: Option<serde_json::Value>,

    // #[serde(rename = "mobile_banner_image")]
    // mobile_banner_image: String,

    // #[serde(rename = "description_html")]
    // description_html: Option<String>,
}

impl RedditUserSubredditList {
    /// Fetch the token owner's subreddit data.
    pub fn fetch_me(reddit_token: &RedditToken) -> AppResult<Self> {
        reddit_token.fetch::<Self>("https://oauth.reddit.com/subreddits/mine/subscriber.json")
    }
}
