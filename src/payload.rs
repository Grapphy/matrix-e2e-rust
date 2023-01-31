use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct LoginPayload {
    pub r#type: String,
    pub identifier: LoginIdentifierSP,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginIdentifierSP {
    pub r#type: String,
    pub user: String,
}

#[derive(Debug, Serialize)]
pub struct KeyPublishPayload {
    pub device_keys: crate::crypto::DeviceKey,
    pub one_time_keys: HashMap<String, crate::crypto::OneTimeKey>,
}