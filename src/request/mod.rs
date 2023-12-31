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
/// use zenn::request::api_request;
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

#[cfg(test)]
mod tests {
    // Using functions from the same file
    use super::*;

    #[tokio::test]
    async fn test_api_request() {
        // Arrange
        let path = "/example";
        let params = "?param=value";

        // Act
        let result = api_request(path, params).await;

        // Assert
        assert!(result.is_ok());
        let response_body = result.unwrap();
        assert!(!response_body.is_empty());
    }
}
