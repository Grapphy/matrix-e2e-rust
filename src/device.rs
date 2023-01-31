use crate::crypto::{DeviceKey, OneTimeKey};
use crate::error::Error;
use crate::http::HTTPBackend;
use std::collections::HashMap;
use vodozemac::olm;

pub struct Device {
    pub user_id: String,
    pub device_id: String,
    pub access_token: String,
    pub homeserver_uri: String,
    pub backend_api: HTTPBackend,
    olm_account: olm::Account,
}

impl Device {
    pub fn new(
        user_id: String,
        device_id: String,
        access_token: String,
        homeserver_uri: String,
    ) -> Self {
        let mut olm_account = olm::Account::new();
        olm_account.generate_one_time_keys(50);
        Device {
            user_id: user_id,
            device_id: device_id,
            access_token: access_token.clone(),
            homeserver_uri: homeserver_uri.clone(),
            backend_api: HTTPBackend::new(homeserver_uri, access_token),
            olm_account: olm_account,
        }
    }

    pub async fn from_login(
        user_id: String,
        password: String,
        homeserver_uri: String,
    ) -> Result<Self, Error> {
        let response =
            HTTPBackend::raw_login(homeserver_uri.clone(), user_id.clone(), password.clone())
                .await?;
        Ok(Device::new(
            user_id,
            response.device_id,
            response.access_token,
            homeserver_uri,
        ))
    }

    pub fn curve25519_key(&self) -> String {
        self.olm_account.curve25519_key().to_base64()
    }

    pub fn ed25519_key(&self) -> String {
        self.olm_account.ed25519_key().to_base64()
    }

    pub async fn publish_keypair(&self) -> Result<i16, Error> {
        let device_key = DeviceKey::new(
            self.device_id.clone(),
            self.user_id.clone(),
            self.curve25519_key(),
            self.ed25519_key(),
        )
        .sign(&self.olm_account);

        let mut one_time_keys: HashMap<String, OneTimeKey> = HashMap::new();
        for (id, curve_key) in self.olm_account.one_time_keys() {
            let otk = OneTimeKey::new(id.to_base64(), curve_key.to_base64()).sign(
                &self.olm_account,
                self.user_id.clone(),
                self.device_id.clone(),
            );
            one_time_keys.insert(
                format!("signed_curve25519:{}", &otk.id[otk.id.len() - 6..]),
                otk,
            );
        }

        let response = self
            .backend_api
            .send_keys(device_key, one_time_keys)
            .await?;
        Ok(response.one_time_key_counts)
    }
}
