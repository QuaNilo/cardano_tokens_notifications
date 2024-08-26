use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub info: Info,
    pub price: Price,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub address: Address,
    pub decimalPlaces: u32,
    pub description: Option<String>,
    pub image: Option<String>,
    pub symbol: Option<String>,
    pub website: Option<String>,
    pub categories: Option<Vec<String>>,
    pub supply: Supply,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub name: Option<String>,
    pub policyId: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supply {
    pub total: Option<String>,
    pub circulating: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub price: f32,
}