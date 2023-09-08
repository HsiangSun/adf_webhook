use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureRsp {
    pub output: OutputObject,
    pub error: Option<ErrorObject>,
    pub status_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputObject {
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorObject {
    pub error_code: String,
    pub message: String,
}
