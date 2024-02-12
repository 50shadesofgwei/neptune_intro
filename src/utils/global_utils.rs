use std::error::Error;
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