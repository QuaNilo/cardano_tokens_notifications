use std::error::Error;
use crate::models::token::TokenResponse;
pub struct TokenScraper {
    url: String,
}

impl TokenScraper {
    pub fn new(url: &str) -> Self {
        TokenScraper { url: url.to_string() }
    }
    pub async fn scrape(&self) -> Result<Vec<TokenResponse>, Box<dyn Error>> {
    // Fetch the JSON response
        let response_text = reqwest::get(&self.url).await?.text().await?;

        // Deserialize the JSON response into TokenResponse
        let token_responses: Vec<TokenResponse> = serde_json::from_str(&response_text)?;
        Ok(token_responses)
    }
}
