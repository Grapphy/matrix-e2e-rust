use crate::error::Error;
use crate::http::HTTPBackend;
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
        Device {
            user_id: user_id,
            device_id: device_id,
            access_token: access_token.clone(),
            homeserver_uri: homeserver_uri.clone(),
            backend_api: HTTPBackend::new(homeserver_uri, access_token),
            olm_account: olm::Account::new(),
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
}
