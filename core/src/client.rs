use std::fmt::Display;

use reqwest::Client;

use crate::parser::ParseError;

const URL: &str = "https://boundvariable.space/communicate";

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    InvalidToken,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RequestError::InvalidToken => write!(f, "Invalid token"),
        }
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(_: reqwest::Error) -> RequestError {
        RequestError::InvalidToken
    }
}

impl From<ParseError> for RequestError {
    fn from(_: ParseError) -> RequestError {
        RequestError::InvalidToken
    }
}

pub struct ICFPCClient {
    auth_token: String,
}

impl ICFPCClient {
    pub fn new(auth_token: String) -> ICFPCClient {
        ICFPCClient { auth_token }
    }

    pub async fn post_message(&self, message: String) -> Result<String, RequestError> {
        let client = Client::new();

        let response = client
            .post(URL)
            .body(message)
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?;

        let text = response.text().await?;
        Ok(text)
    }
}
