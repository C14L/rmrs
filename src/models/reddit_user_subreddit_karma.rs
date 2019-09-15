extern crate serde_json;

use serde::{Serialize, Deserialize};

use crate::helpers::AppResult;
use crate::models::reddit_token::RedditToken;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RedditUserSubredditKarmaList {
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "data")]
    pub data: Vec<RedditUserSubredditKarmaData>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RedditUserSubredditKarmaData {
    #[serde(rename = "sr", default = "__INVALID__".to_string())]
    pub sr: String,

    #[serde(rename = "comment_karma")]
    pub comment_karma: i32,

    #[serde(rename = "link_karma")]
    pub link_karma: i32,
}

impl RedditUserSubredditKarmaList {
    /// Fetch the token owner's reddit data.
    pub fn fetch_me(reddit_token: &RedditToken) -> AppResult<Self> {
        reddit_token.fetch::<Self>("https://oauth.reddit.com/api/v1/me/karma.json")
    }
}
