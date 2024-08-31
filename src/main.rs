mod db;
mod scraper;
mod models;
use scraper::token_scraper::TokenScraper;
use tokio;
use crate::db::sqlite_client::Db;


#[tokio::main]
async fn main() {
    // Initialize TokenScraper and scrape tokens
    let token_scraper = TokenScraper::new("https://api.muesliswap.com/list");
    let database = Db::initiate_pool("sqlite://my_database.db").await;
    let db = match database {
        Ok(db) => db,
        Err(e) => {
            println!("Error occurred while initiating the database: {}", e);
            return;
        }
    };
    db.create_schema().await.expect("Failed to initialize database");

    // Scrape tokens
    let tokens = match token_scraper.scrape().await {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Error occurred while scraping tokens: {}", e);
            return;
        }
    };

    // Insert token information into the database
    if let Err(e) = db.insert_token_info(tokens).await {
        println!("Error occurred while inserting token info: {}", e);
    }

    println!("Finished successfully!");
}
