#[derive(Debug)]
pub enum Error {
    ApiError(String),
    HTTPInternalError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ApiError(resp) => write!(f, "Error response: {:?}", resp),
            Error::HTTPInternalError(resp) => write!(f, "HTTP Request failed: {:?}", resp),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPInternalError(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::HTTPInternalError(String::from(format!("Invalid response: {}", e.to_string())))
    }
}
