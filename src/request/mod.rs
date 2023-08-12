use crate::constants::API_BASE_URL;
use reqwest;

/// Function to make an API request.
///
/// # Arguments
///
/// * `path` - Path of the API endpoint.
/// * `params` - Query parameters.
///
/// Sends a request and returns the response as a string.
///
/// # Returns
///
/// Returns a `Result` containing the response body as a string or a `reqwest::Error` if the request fails.
///
/// # Example
///
/// ```rust
/// use my_api_client::api_request;
///
/// #[tokio::main]
/// async fn main() {
///     let path = "/example";
///     let params = "?param=value";
///     
///     match api_request(path, params).await {
///         Ok(response) => {
///             println!("Response: {}", response);
///         }
///         Err(err) => {
///             eprintln!("Error: {}", err);
///         }
///     }
/// }
/// ```
pub async fn api_request(path: &str, params: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}{}{}", API_BASE_URL, path, params);

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    Ok(body)
}
