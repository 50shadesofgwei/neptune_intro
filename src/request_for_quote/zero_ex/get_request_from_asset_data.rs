use reqwest::{self, Client, Response};
use serde::Deserialize;
use std::error::Error;
use dotenv::dotenv;
use std::env;
use crate::utils::api_error_handling_utils::{is_valid_api_response, ApiError};
use crate::utils::global_types::AssetToSellData;
use crate::utils::global_utils::deserialize_json_response;
use super::{zero_ex_utils::ZERO_EX_API_ADDRESS, zero_ex_types::ZeroExAPIResponseData};

#[derive(Debug, Deserialize)]
pub struct ZeroExAPIError {
    pub message: String,
}

pub struct ZeroExQuoter {
    api_key: String,
}

impl ZeroExQuoter {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let api_key: String = env::var("ZERO_EX_API_KEY")
        .map_err(|e| format!("Couldn't read ZERO_EX_API_KEY from .env ({})", e))?;

        Ok(Self {
            api_key,
        })
    }

    pub async fn get_zero_ex_quote_from_asset_data(
        &self,
        asset_data: AssetToSellData,
    ) -> Result<ZeroExAPIResponseData, Box<dyn Error>> {
        let client: Client = Client::new();
        let response: Response = client
            .get(ZERO_EX_API_ADDRESS)
            .query(&[
                ("buyToken", &asset_data.buy_token_address),
                ("sellToken", &asset_data.sell_token_address),
                ("sellAmount", &asset_data.sell_token_amount),
            ])
            .header("0x-api-key", &self.api_key)
            .send()
            .await
            .map_err(ApiError::from)?;

        let validated_response: Response = is_valid_api_response(response).await?;
        let zero_ex_api_response: ZeroExAPIResponseData = deserialize_json_response(validated_response).await?;
    
        Ok(zero_ex_api_response)
    }
    
    }

