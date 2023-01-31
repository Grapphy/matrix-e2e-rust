use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errcode: String,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub device_id: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyUploadResponse {
    pub one_time_key_counts: i16,
}
