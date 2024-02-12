use std::error::Error;
use utils::global_types::AssetToSellData;
use crate::request_for_quote::zero_ex::get_request_from_asset_data::ZeroExQuoter;

mod request_for_quote;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rfq_object: AssetToSellData = AssetToSellData {
        sell_token_address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
        buy_token_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
        sell_token_amount: "1000000000000000000".to_string(),
    };

    let quoter: ZeroExQuoter = ZeroExQuoter::new()?;
    let quote: request_for_quote::zero_ex::zero_ex_types::ZeroExAPIResponseData = quoter.get_zero_ex_quote_from_asset_data(rfq_object).await?;

    println!("{:?}", quote);

    Ok(())
}
