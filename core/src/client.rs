use reqwest::Client;

const URL: &str = "https://boundvariable.space/communicate";

pub struct ICFPCClient {
    auth_token: String,
}

impl ICFPCClient {
    pub fn new(auth_token: String) -> ICFPCClient {
        ICFPCClient { auth_token }
    }

    pub async fn post_message(&self, message: String) -> Result<String, reqwest::Error> {
        let client = Client::new();

        let response = client
            .post(URL)
            .body(message)
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?;

        response.text().await
    }
}
