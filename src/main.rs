mod db;
mod scraper;
mod models;
use scraper::token_scraper::TokenScraper;
use tokio;
use crate::db::mongodb_client::Db;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize TokenScraper and scrape tokens
    let token_scraper = TokenScraper::new("https://api.muesliswap.com/list");

    // Initialize the database connection
    let database = Db::initiate_collection("mongodb://localhost:27017").await?;

    // Scrape tokens
    let tokens = match token_scraper.scrape().await {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error occurred while scraping tokens: {}", e);
            return Err(Box::new(e));
        }
    };

    // Insert token information into the database
    if let Err(e) = database.insert_token_info(tokens).await {
        eprintln!("Error occurred while inserting token info: {}", e);
        return Err(Box::new(e));
    }

    Ok(())
}