use std::error::Error;
use rusqlite::{Connection, Result};
use crate::models::token::TokenResponse;
use std::collections::HashMap;
pub struct Db {
    conn: Connection,
}

impl Db {
    // Create tables and schema
    pub async fn create_schema(&self) -> Result<(), Box<dyn Error>> {

        // Define and execute schema creation queries
        self.conn.execute("CREATE TABLE IF NOT EXISTS price (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token_id TEXT UNIQUE NOT NULL,
                price REAL NOT NULL
            )", ())?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS address (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token_id TEXT UNIQUE NOT NULL,
                name TEXT,
                policy_id TEXT
            )", ())?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS supply (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token_id TEXT UNIQUE NOT NULL,
                total TEXT,
                circulating TEXT
        )", ())?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS info (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token_id TEXT UNIQUE NOT NULL,
                decimal_places INTEGER,
                description TEXT,
                image TEXT,
                symbol TEXT,
                website TEXT,
                categories TEXT, -- Store as a JSON string
                supply_id INTEGER,
                status TEXT,
                FOREIGN KEY (supply_id) REFERENCES supply(id)
        )", ())?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS token_info (
                id TEXT PRIMARY KEY,
                info_id INTEGER NOT NULL,
                FOREIGN KEY (info_id) REFERENCES info(id)
        )", ())?;

        Ok(())
    }

        // Initialize the database connection
    pub async fn initiate_pool(database_url: &str) -> Result<(Db)> {
        // Use block_in_place to handle synchronous code in an async context
        let conn = tokio::task::block_in_place(|| Connection::open("tokens.db"))?;

        // Create Db instance
        let db = Db { conn };
        Ok(db)
    }

    // Insert token information into the database
    pub async fn insert_token_info(&self, tokens: Vec<TokenResponse>) -> Result<(), Box<dyn Error>> {
        for token in &tokens {
            let policy_id = token.info.address.policy_id.as_deref().unwrap_or_default();
            let description = token.info.description.as_deref().unwrap_or_default();
            let image = token.info.image.as_deref().unwrap_or_default();
            let symbol = token.info.symbol.as_deref().unwrap_or_default();
            let website = token.info.website.as_deref().unwrap_or_default();
            let status = token.info.status.as_deref().unwrap_or_default();
                    // Convert Vec<String> to JSON string
            let categories_json = token.info.categories
                .as_ref()
                .map(|categories| serde_json::to_string(categories).unwrap_or_default())
                .unwrap_or_default();

            // Insert Price
            self.conn.execute("INSERT INTO price (token_id, price) VALUES (?, ?)",
                              (token.info.address.policy_id.as_deref().unwrap_or_default(),
                               token.price.price))
                .expect("Failed to insert into 'price'");

            // Insert Address
            self.conn.execute("INSERT INTO address (token_id, name, policy_id) VALUES (?, ?, ?)",
                              (token.info.address.policy_id.as_deref().unwrap_or_default(),
                               &token.info.address.name,
                               token.info.address.policy_id.as_deref()))
                .expect("Failed to insert into 'address'");


            // Insert Supply
            self.conn.execute("INSERT INTO supply (token_id, total, circulating) VALUES (?, ?, ?)",
                              (token.info.address.policy_id.as_deref().unwrap_or_default(),
                               token.info.supply.circulating.as_deref(),
                               token.info.supply.total.as_deref()))
                .expect("Failed to insert into 'supply'");

             // Insert Info
            self.conn.execute(
                "INSERT INTO info (token_id, decimal_places, description, image, symbol, website, categories, supply_id, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                (
                    policy_id,
                    token.info.decimal_places, // `u32` can be used directly
                    description,
                    image,
                    symbol,
                    website,
                    categories_json,
                    0, // Supply ID, adjust as needed
                    status,
                ),
            )?;

            // Insert TokenResponse
            self.conn.execute("INSERT INTO token_info (id, info_id) VALUES (?, ?)",
                              (token.info.address.policy_id.as_deref().unwrap_or_default(),
                               token.info.address.policy_id.as_deref().unwrap_or_default()))
                .expect("Failed to insert into TokenResponse");
        }

        println!("Inserted {} documents", tokens.len());
        Ok(())
    }
}
