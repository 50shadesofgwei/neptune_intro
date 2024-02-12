use reqwest::{Response, Error as ReqwestError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    msg: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ApiError {}

impl From<ReqwestError> for ApiError {
    fn from(error: ReqwestError) -> Self {
        ApiError {
            msg: format!("Network error: {}", error),
        }
    }
}

impl From<String> for ApiError {
    fn from(error: String) -> Self {
        ApiError { msg: error }
    }
}

pub async fn is_valid_api_response(res: Response) -> Result<Response, Box<dyn Error>> {
    if !res.status().is_success() {
        let error_message: String = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(Box::new(ApiError::from(format!("Error response from API: {}", error_message))))
    } else {
        Ok(res)
    }
}