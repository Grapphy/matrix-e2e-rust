use crate::crypto::DeviceKey;
use crate::device::Device;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct KeyExchangeData {
    pub algorithm: String,
    pub room_id: String,
    pub session_id: String,
    pub session_key: String,
}

#[derive(Debug, Serialize)]
pub struct KeyExchangeEvent {
    pub sender: String,
    pub sender_device: String,
    pub keys: HashMap<String, String>,
    pub recipient: String,
    pub recipient_keys: HashMap<String, String>,
    pub r#type: String,
    pub content: KeyExchangeData,
}

#[derive(Debug, Serialize)]
pub struct RoomEncryptedOLM {
    pub r#type: i8,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct OlmExchange {
    pub algorithm: String,
    pub sender_key: String,
    pub ciphertext: HashMap<String, RoomEncryptedOLM>,
}

impl OlmExchange {
    pub fn new(
        sender_device: &Device,
        recipient_device: &DeviceKey,
        megolm_session: &vodozemac::megolm::GroupSession,
        olm_session: &mut vodozemac::olm::Session,
        room_id: String,
    ) -> Self {
        let key_exchange_data = KeyExchangeData {
            algorithm: String::from("m.megolm.v1.aes-sha2"),
            room_id: room_id,
            session_id: megolm_session.session_id(),
            session_key: megolm_session.session_key().to_base64(),
        };

        let key_exchange_event = KeyExchangeEvent {
            sender: sender_device.user_id.clone(),
            sender_device: sender_device.device_id.clone(),
            keys: HashMap::from([(String::from("ed25519"), sender_device.ed25519_key())]),
            recipient: recipient_device.user_id.clone(),
            recipient_keys: HashMap::from([(
                String::from("ed25519"),
                recipient_device.keys[format!("ed25519:{}", recipient_device.device_id)]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )]),
            r#type: String::from("m.room_key"),
            content: key_exchange_data,
        };

        let json_payload = serde_json::to_string(&key_exchange_event).unwrap();
        let encrypted_payload = olm_session.encrypt(json_payload).to_parts().1;

        let room_olm = RoomEncryptedOLM {
            r#type: 0,
            body: encrypted_payload,
        };

        let ciphertext = HashMap::from([(
            recipient_device.keys[format!("curve25519:{}", recipient_device.device_id)]
                .as_str()
                .unwrap()
                .to_owned(),
            room_olm,
        )]);
        OlmExchange {
            algorithm: String::from("m.olm.v1.curve25519-aes-sha2"),
            sender_key: sender_device.curve25519_key(),
            ciphertext: ciphertext,
        }
    }
}
