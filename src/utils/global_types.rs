use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetToSellData {
    pub sell_token_address: &'static str,
    pub buy_token_address: &'static str,
    pub sell_token_amount: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxData {
    pub to: String,
    pub value: String,
    pub data: String,
}

pub const TEST_RFQ_OBJECT: AssetToSellData = AssetToSellData {
    sell_token_address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    buy_token_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    sell_token_amount: "1000000000000000000",
};