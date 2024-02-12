use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZeroExAPIResponseData {
    buy_token_address: String,
    sell_token_address: String,
    sell_amount: String,
    buy_amount: String,
    to: String,
    value: String,
    data: String,
}