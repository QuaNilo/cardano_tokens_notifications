use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use mongodb::error::Result;
use crate::models::token::TokenResponse;

pub struct Db {
    uri: String,
    collection: Option<Collection<TokenResponse>>,
}

impl Db {
    pub async fn initiate_collection(uri: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("cardano_native_tokens");
        let collection = database.collection::<TokenResponse>("token_info");

        Ok(Db {
            uri: uri.to_string(),
            collection: Some(collection),
        })
    }

    pub async fn insert_token_info(&self, tokens: Vec<TokenResponse>) -> Result<()> {
        if let Some(collection) = &self.collection {
            match collection.insert_many(tokens, None).await {
                Ok(result) => {
                    println!("Inserted document IDs: {:?}", result.inserted_ids);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error inserting documents: {:?}", e);
                    Err(e)
                }
            }
        } else {
            eprintln!("Collection not initialized");
            Ok(())
        }
    }
}
