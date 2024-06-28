use core::client::ICFPCClient;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let auth_token = "5b4a264f-5e00-433c-ac1b-1f9a8b30f161".to_string();
    let client = ICFPCClient::new(auth_token);

    let message = "S'%4}).$%8".to_string();
    let response_message = client.post_message(message).await?;
    eprintln!("---");
    eprintln!("{}", response_message);
    eprintln!("---");

    Ok(())
}
