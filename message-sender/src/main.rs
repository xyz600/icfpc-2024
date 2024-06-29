use clap::Parser;
use core::{client::ICFPCClient, parser::icfpstring::ICFPString};
use std::fs;
use std::path::PathBuf;

/// このプログラムはコマンドライン引数からファイルパスを受け取り、その内容を出力します。
#[derive(Parser, Debug)]
#[command(name = "file_reader")]
#[command(about = "A simple file reader")]
struct Args {
    /// ファイルパス
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long)]
    encode: bool,
}

fn get_content(path: &PathBuf) -> Result<String, anyhow::Error> {
    fs::read_to_string(path).map_err(|e| e.into())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let contents = get_content(&args.file)?;
    let auth_token = "5b4a264f-5e00-433c-ac1b-1f9a8b30f161".to_string();
    let client = ICFPCClient::new(auth_token);

    let message = if args.encode {
        let s = ICFPString::from_encoded_str(&contents.as_str())?;
        let encoded = s.to_string()?.into_iter().collect::<String>();
        format!("S{}", encoded)
    } else {
        contents
    };

    let response_message = client.post_message(message).await?;
    eprintln!("---");
    eprintln!("{}", response_message);
    eprintln!("---");
    Ok(())
}
