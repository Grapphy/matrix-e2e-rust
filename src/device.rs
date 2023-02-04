use crate::crypto::{DeviceKey, MegolmSession, OlmExchange, OneTimeKey};
use crate::error::Error;
use crate::http::HTTPBackend;
use std::collections::HashMap;
use vodozemac::megolm;
use vodozemac::olm;

pub struct Device {
    pub user_id: String,
    pub device_id: String,
    pub access_token: String,
    pub homeserver_uri: String,
    pub backend_api: HTTPBackend,
    olm_account: olm::Account,
    megolm_sessions: HashMap<String, megolm::GroupSession>,
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
            megolm_sessions: HashMap::new(),
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
        Ok(response.one_time_key_counts.signed_curve25519.unwrap_or(0))
    }

    pub async fn create_megolm_session(
        &self,
        room_id: String,
        user_id: String,
        recipient_device_id: String,
    ) -> Result<MegolmSession, Error> {
        let queried_keys = self.backend_api.query_keys(user_id.clone()).await?;
        let recipient_device: &DeviceKey =
            &queried_keys.device_keys[&user_id][&recipient_device_id];

        let claimed_otks = self
            .backend_api
            .claim_otk(user_id.clone(), recipient_device_id.clone())
            .await?;
        let user_otk = claimed_otks.one_time_keys[&user_id][&recipient_device_id]
            .values()
            .last()
            .unwrap()
            .curve25519_key
            .clone();

        let outbound_group_session = self
            .create_olm_exchange(recipient_device, user_otk, room_id.clone())
            .await?;
        Ok(MegolmSession::new(room_id, outbound_group_session))
    }

    pub async fn send_encrypted_message(
        &self,
        megolm_session: &mut MegolmSession,
        content: &str,
    ) -> Result<bool, Error> {
        let message =
            megolm_session.create_message(self.curve25519_key(), self.device_id.clone(), content);

        self.backend_api
            .send_message(megolm_session.room_id.clone(), message)
            .await?;

        Ok(true)
    }

    async fn create_olm_exchange(
        &self,
        recipient_device: &DeviceKey,
        user_otk: String,
        room_id: String,
    ) -> Result<megolm::GroupSession, Error> {
        let outbound_group_session = megolm::GroupSession::new(megolm::SessionConfig::version_1());

        let recipient_curve25519 = vodozemac::Curve25519PublicKey::from_base64(
            recipient_device.keys[format!("curve25519:{}", recipient_device.device_id)]
                .as_str()
                .unwrap(),
        )
        .unwrap();

        let recipient_otk = vodozemac::Curve25519PublicKey::from_base64(&user_otk).unwrap();

        let mut outbound_olm_session = self.olm_account.create_outbound_session(
            olm::SessionConfig::version_1(),
            recipient_curve25519,
            recipient_otk,
        );

        let olm_exchange_payload = OlmExchange::new(
            self,
            recipient_device,
            &outbound_group_session,
            &mut outbound_olm_session,
            room_id,
        );

        self.backend_api
            .send_olm(
                recipient_device.user_id.clone(),
                recipient_device.device_id.clone(),
                olm_exchange_payload,
            )
            .await?;
        Ok(outbound_group_session)
    }
}
