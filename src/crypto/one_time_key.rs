use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct OneTimeKey {
    #[serde(skip)]
    pub id: String,
    #[serde(rename = "key")]
    pub curve25519_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<HashMap<String, HashMap<String, String>>>,
}

impl OneTimeKey {
    pub fn new(id: String, curve25519_key: String) -> Self {
        OneTimeKey {
            id: id,
            curve25519_key: curve25519_key,
            signatures: None,
        }
    }

    pub fn sign(
        mut self,
        olm: &vodozemac::olm::Account,
        user_id: String,
        device_id: String,
    ) -> Self {
        let signature = olm.sign(&serde_json::to_string(&self).unwrap());

        let mut signed_map = HashMap::new();
        signed_map.insert(
            user_id,
            HashMap::from([(format!("ed25519:{}", device_id), signature.to_base64())]),
        );

        self.signatures = Some(signed_map);
        self
    }
}
