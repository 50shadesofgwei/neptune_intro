use reqwest::{self, Client, Response};
use serde::Deserialize;
use std::error::Error;
use std::sync::mpsc::Sender;
use dotenv::dotenv;
use crate::utils::api_error_handling_utils::{is_valid_api_response, ApiError};
use crate::utils::global_types::{AssetToSellData, TxData};
use crate::utils::global_utils::{deserialize_json_response, get_environment_variable, EnvironmentVariable};
use crate::utils::events::emitter::{get_emitter};
use super::{zero_ex_utils::ZERO_EX_API_ADDRESS, zero_ex_types::ZeroExAPIResponseData};

#[derive(Debug, Deserialize)]
pub struct ZeroExAPIError {
    pub message: String,
}

pub struct ZeroExQuoter {
    api_key: String,
    emitter: Sender<TxData>
}

impl ZeroExQuoter {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let api_key: String = get_environment_variable(EnvironmentVariable::ZeroExApiKey)?;
        let emitter: Sender<TxData> = get_emitter();

        Ok(Self {
            api_key,
            emitter
        })
    }

    pub fn emit_tx_data(&self, data_to_emit: TxData) -> Result<(), Box<dyn Error>> {
        self.emitter.send(data_to_emit).map_err(|e| e.into())
    }

    pub async fn get_tx_data_from_asset_data(
        &self,
        asset_data: AssetToSellData,
    ) -> Result<TxData, Box<dyn Error>> {
        let quote: ZeroExAPIResponseData = self.get_zero_ex_quote_from_asset_data(asset_data).await?;
        let tx_data: TxData = self.get_tx_data_from_api_response(quote);
        
        Ok(tx_data)
    }

    async fn get_zero_ex_quote_from_asset_data(
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
    
    fn get_tx_data_from_api_response(
        &self,
        response_data: ZeroExAPIResponseData
    ) -> TxData {
        TxData {
            to: response_data.to,
            value: response_data.value,
            data: response_data.data,
        }
    }
    
}

