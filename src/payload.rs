use serde::Serialize;

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
