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
