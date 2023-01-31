use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct DeviceKey {
    pub algorithms: Vec<String>,
    pub device_id: String,
    pub user_id: String,
    pub keys: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<HashMap<String, HashMap<String, String>>>,
}

impl DeviceKey {
    pub fn new(
        device_id: String,
        user_id: String,
        curve25519_key: String,
        ed25519_key: String,
    ) -> Self {
        let algorithms = vec![
            String::from("m.olm.curve25519-aes-sha256"),
            String::from("m.megolm.v1.aes-sha"),
        ];

        let keys: serde_json::Value = serde_json::json!({
            format!("curve25519:{}", device_id): curve25519_key,
            format!("ed25519:{}", device_id): ed25519_key,
        });

        DeviceKey {
            algorithms: algorithms,
            device_id: device_id,
            user_id: user_id,
            keys: keys,
            signatures: None,
        }
    }

    pub fn sign(mut self, olm: &vodozemac::olm::Account) -> Self {
        let signature = olm.sign(&serde_json::to_string(&self).unwrap());

        let mut signed_map = HashMap::new();
        signed_map.insert(
            self.user_id.clone(),
            HashMap::from([(format!("ed25519:{}", self.device_id), signature.to_base64())]),
        );

        self.signatures = Some(signed_map);
        self
    }
}
