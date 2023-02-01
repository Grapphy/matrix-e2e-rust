use serde::Serialize;
use vodozemac::megolm;

pub struct MegolmSession {
    pub room_id: String,
    pub ratchet: megolm::GroupSession,
}

#[derive(Debug, Serialize)]
pub struct MegolmMessage {
    pub algorithm: String,
    pub sender_key: String,
    pub ciphertext: String,
    pub session_id: String,
    pub device_id: String,
}

#[derive(Debug, Serialize)]
pub struct PlainTextContent {
    pub msgtype: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct PlainTextMessage {
    pub r#type: String,
    pub content: PlainTextContent,
    pub room_id: String,
}

impl MegolmSession {
    pub fn new(room_id: String) -> Self {
        MegolmSession {
            room_id: room_id,
            ratchet: megolm::GroupSession::new(megolm::SessionConfig::version_1()),
        }
    }

    pub fn create_message(
        &mut self,
        sender_key: String,
        device_id: String,
        content: &str,
    ) -> MegolmMessage {
        let message_payload = PlainTextMessage {
            r#type: String::from("m.room.message"),
            content: PlainTextContent {
                msgtype: String::from("m.text"),
                body: content.to_owned(),
            },
            room_id: self.room_id.clone(),
        };
        let json_string = serde_json::to_string(&message_payload).unwrap();
        let ciphertext = self.ratchet.encrypt(json_string).to_base64();

        MegolmMessage {
            algorithm: String::from("m.megolm.v1.aes-sha2"),
            sender_key: sender_key,
            ciphertext: ciphertext,
            session_id: self.ratchet.session_id(),
            device_id: device_id,
        }
    }
}
