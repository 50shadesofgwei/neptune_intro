use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetToSellData {
    pub sell_token_address: String,
    pub buy_token_address: String,
    pub sell_token_amount: String
}