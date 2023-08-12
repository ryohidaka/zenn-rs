use reqwest::Error as ReqwestError;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// Struct to hold parameters for fetching articles
pub struct ArticleSearchParams {
    pub username: String, // The username associated with the articles
    pub count: u32,       // The number of articles to fetch (default: 96)
    pub order: String,    // The order in which to fetch the articles (default: "latest")
}

/// Implementation of the Default trait for ArticleSearchParams
///
/// This implementation provides default values for the fields of
/// ArticleSearchParams, including an empty username, a count of 96,
/// and an order of "latest".
impl Default for ArticleSearchParams {
    fn default() -> Self {
        ArticleSearchParams {
            username: String::default(),
            count: 96,
            order: "latest".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub article_type: String,
    pub body_letters_count: u32,
    pub body_updated_at: String,
    pub comments_count: u32,
    pub emoji: String,
    pub id: u32,
    pub is_suspending_private: bool,
    pub liked_count: u32,
    pub path: String,
    pub pinned: bool,
    pub post_type: String,
    pub publication: Option<String>,
    pub published_at: String,
    pub slug: String,
    pub source_repo_updated_at: Option<String>,
    pub title: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar_small_url: String,
    pub id: u32,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub articles: Vec<Article>,
}

#[derive(Debug)]
pub enum FetchError {
    RequestError(ReqwestError),
    JsonError(JsonError),
}
