use serde::{Serialize, Deserialize};
use serde_json;

use crate::helpers::AppResult;
use crate::models::reddit_token::RedditToken;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RedditUser {
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

impl RedditUser {
    /// Fetch user data for authenticated user from Reddit API.
    pub fn fetch_me(reddit_token: &RedditToken) -> AppResult<Self> {
        reddit_token.fetch::<Self>("https://oauth.reddit.com/api/v1/me.json")
    }
}
