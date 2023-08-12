use crate::{request::api_request, types::ArticleSearchParams};

/// Fetches a list of articles based on specified search parameters.
///
/// This function is used to retrieve a list of articles based on the provided search conditions.
/// It constructs the API endpoint path and query parameters using the information from `params`.
/// The `api_request` function is then called to make the API request and retrieve the data for
/// the article list.
///
/// # Arguments
///
/// * `params` - A struct containing the search parameters for fetching articles.
///
/// # Returns
///
/// A `Result` containing the fetched article data as a string or a `reqwest::Error` in case of failure.
///
/// # Example
///
/// ```rust
/// use zenn::articles::fetch_articles;
/// use zenn::types::ArticleSearchParams;
///
/// #[tokio::main]
/// async fn main() {
///     let search_params = ArticleSearchParams {
///         username: "example_user".to_string(),
///         count: 10,
///         order: "latest".to_string(),
///     };
///
///     let result = fetch_articles(search_params).await;
///     match result {
///         Ok(data) => println!("Fetched articles: {}", data),
///         Err(err) => eprintln!("Error fetching articles: {:?}", err),
///     }
/// }
/// ```
pub async fn fetch_articles(params: ArticleSearchParams) -> Result<String, reqwest::Error> {
    let path = "/articles";
    let params = format!(
        "?username={}&count={}&order={}",
        params.username, params.count, params.order
    );
    api_request(path, &params).await
}