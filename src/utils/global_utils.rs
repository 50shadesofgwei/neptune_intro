use std::{error::Error, env};
use serde::Deserialize;
use reqwest::Response;
use crate::utils::api_error_handling_utils::ApiError;

pub async fn deserialize_json_response<T>(res: Response) -> Result<T, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    res.json::<T>().await.map_err(|e| {
        let error_msg: String = format!("Error deserializing response: {}", e);
        Box::new(ApiError::from(error_msg)) as Box<dyn Error>
    })
}

pub enum EnvironmentVariable {
    ZeroExApiKey,
    ExecutorWalletPrivateKey,
}

impl EnvironmentVariable {
    fn as_str(&self) -> &'static str {
        match self {
            EnvironmentVariable::ZeroExApiKey => "ZERO_EX_API_KEY",
            EnvironmentVariable::ExecutorWalletPrivateKey => "EXECUTOR_WALLET_PRIVATE_KEY",
        }
    }
}

pub fn get_environment_variable(env_var: EnvironmentVariable) -> Result<String, Box<dyn Error>> {
    let key: &str = env_var.as_str();
    env::var(key).map_err(|e| e.into())
}