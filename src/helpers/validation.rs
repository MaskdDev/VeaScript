use url::Url;

/// Validate a URL.
pub fn url(url: &str, field: &str) -> Result<(), String> {
    // Check if URL is valid
    if let Ok(_) = Url::parse(url) {
        Ok(())
    } else {
        Err(format!("Invalid URL provided for {}: {}.", field, url))
    }
}
