use std::error::Error;
use utils::global_types::{TEST_RFQ_OBJECT, TxData};
use crate::request_for_quote::zero_ex::get_request_from_asset_data::ZeroExQuoter;

mod request_for_quote;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let quoter: ZeroExQuoter = ZeroExQuoter::new()?;
    let quote: TxData = quoter.get_tx_data_from_asset_data(TEST_RFQ_OBJECT).await?;
    
    println!("{:?}", quote);
    Ok(())
}
