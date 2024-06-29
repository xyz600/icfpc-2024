use clap::Parser;
use core::client::ICFPCClient;
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
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // ファイルの内容を読み込む
    match fs::read_to_string(&args.file) {
        Ok(contents) => {
            let auth_token = "5b4a264f-5e00-433c-ac1b-1f9a8b30f161".to_string();
            let client = ICFPCClient::new(auth_token);

            let response_message = client.post_message(contents).await?;
            eprintln!("---");
            eprintln!("{}", response_message);
            eprintln!("---");
            Ok(())
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            Err(error.into())
        }
    }
}
