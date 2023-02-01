use serde::Deserialize;
use std::collections::HashMap;

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
    pub one_time_key_counts: OneTimeKeyCounts,
}

#[derive(Debug, Deserialize)]
pub struct OneTimeKeyCounts {
    pub signed_curve25519: Option<i16>,
    pub curve25519: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct RequestDeviceKeyResponse {
    pub device_keys: HashMap<String, HashMap<String, crate::crypto::DeviceKey>>,
}
