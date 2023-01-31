use crate::error::Error;
use crate::payload::{LoginIdentifierSP, LoginPayload};
use crate::response::{ErrorResponse, LoginResponse};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub struct Route {
    method: reqwest::Method,
    path: String,
}

pub struct HTTPBackend {
    pub http_client: reqwest::Client,
    pub homeserver_uri: String,
    pub access_token: String,
}

impl Route {
    pub fn new(method: &str, path: &str) -> Self {
        Route {
            method: match method {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "PUT" => reqwest::Method::PUT,
                _ => reqwest::Method::default(),
            },
            path: String::from(path),
        }
    }
}

impl HTTPBackend {
    pub fn new(homeserver_uri: String, access_token: String) -> Self {
        HTTPBackend {
            http_client: reqwest::Client::new(),
            homeserver_uri: homeserver_uri,
            access_token: access_token,
        }
    }

    pub async fn request<S: Serialize, D: DeserializeOwned>(
        &self,
        route: Route,
        data: Option<S>,
    ) -> Result<D, Error> {
        let method = route.method;
        let url = format!("{}{}", self.homeserver_uri, route.path);
        let mut request = self.http_client.request(method.clone(), url);

        if method == reqwest::Method::POST || method == reqwest::Method::PUT {
            request = request.json(&data);
        }

        let response = request.send().await?;
        if response.status().is_success() {
            let rjson = response.json::<D>().await?;
            return Ok(rjson);
        }
        let ejson = response.json::<ErrorResponse>().await?;
        Err(Error::ApiError(ejson.error))
    }

    pub async fn raw_login(
        homeserver_uri: String,
        username: String,
        password: String,
    ) -> Result<LoginResponse, Error> {
        let endpoint = format!("{}/_matrix/client/r0/login", homeserver_uri);
        let client = reqwest::Client::new();
        let response: LoginResponse = client
            .post(endpoint)
            .json(&LoginPayload {
                r#type: String::from("m.login.password"),
                identifier: LoginIdentifierSP {
                    r#type: String::from("m.id.user"),
                    user: username,
                },
                password: password,
            })
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
