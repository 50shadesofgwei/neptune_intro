#[derive(Debug, Serialize, Deserialize)]
pub struct ZeroExAPIResponseData {
    buy_token_address: String,
    sell_token_address: String,
    sell_amount: String,
    buy_amount: String,
    to: String,
    value: String,
    data: String,
}