use std::error::Error;
use utils::global_types::{TEST_RFQ_OBJECT};
use crate::request_for_quote::zero_ex::get_request_from_asset_data::ZeroExQuoter;
use request_for_quote::zero_ex::zero_ex_types::ZeroExAPIResponseData;


mod request_for_quote;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let quoter: ZeroExQuoter = ZeroExQuoter::new()?;
    let quote: ZeroExAPIResponseData = quoter.get_zero_ex_quote_from_asset_data(TEST_RFQ_OBJECT).await?;
    
    println!("{:?}", quote);
    Ok(())
}
