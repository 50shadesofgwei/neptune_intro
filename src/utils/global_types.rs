#[derive(Debug, Serialize, Deserialize)]
pub struct AssetToSellData {
    sell_token_address: String,
    buy_token_address: String,
    sell_token_amount: String
}