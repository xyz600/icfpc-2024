use clap::{Parser, Subcommand};
use core::parser::ast::{parse, NodeType};
use core::{client::ICFPCClient, parser::icfpstring::ICFPString};
use std::fs;
use std::path::PathBuf;

/// このプログラムはコマンドライン引数からファイルパスを受け取り、その内容を出力します。
#[derive(Parser, Debug, Clone)]
#[command(name = "file_reader")]
#[command(about = "A simple file reader")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Lambdaman,
    LambdamanGet {
        #[arg(short, long)]
        problem_id: String,
    },
    LambdamanSubmit {
        #[arg(short, long)]
        problem_id: String,

        #[arg(short, long)]
        filepath: PathBuf,
    },
    Spaceship,
    SpaceshipGet {
        #[arg(short, long)]
        problem_id: String,
    },
    Echo {
        #[arg(short, long)]
        message: String,
    },
    Scoreboard,
    LanguageTest,
    Efficiency,
    EfficiencyGet {
        #[arg(short, long)]
        problem_id: String,
    },
    EfficiencySubmit {
        #[arg(short, long)]
        problem_id: String,

        #[arg(short, long)]
        filepath: PathBuf,
    },
    D3,
    D3Get {
        #[arg(short, long)]
        problem_id: String,
    },
}

fn read_content(path: &PathBuf) -> Result<String, anyhow::Error> {
    fs::read_to_string(path).map_err(|e| e.into())
}

fn encode(contents: String) -> Result<String, anyhow::Error> {
    let s = ICFPString::from_encoded_str(&contents.as_str())?;
    let encoded = s.to_string()?.into_iter().collect::<String>();
    Ok(format!("S{}", encoded))
}

fn decode(contents: String) -> Result<String, anyhow::Error> {
    let decoded_message = parse(contents)?;
    match decoded_message.node_type {
        NodeType::String(s) => Ok(s.iter().collect::<String>()),
        _ => Err(anyhow::anyhow!("Invalid message")),
    }
}

fn select_content(command: Commands) -> Result<String, anyhow::Error> {
    match command {
        Commands::Spaceship => Ok("get spaceship".to_string()),
        Commands::SpaceshipGet { problem_id } => Ok(format!("get spaceship{}", problem_id)),
        Commands::Echo { message } => Ok(format!("get echo {}", message)),
        Commands::Scoreboard => Ok("get scoreboard".to_string()),
        Commands::LanguageTest => Ok("get language_test".to_string()),
        Commands::Efficiency => Ok("get efficiency".to_string()),
        Commands::EfficiencyGet { problem_id } => Ok(format!("get efficiency{}", problem_id)),
        Commands::EfficiencySubmit {
            problem_id,
            filepath,
        } => {
            let contents = read_content(&filepath)?;
            Ok(format!("solve efficiency{} {}", problem_id, contents))
        }
        Commands::D3 => Ok("get 3d".to_string()),
        Commands::D3Get { problem_id } => Ok(format!("get 3d{}", problem_id)),
        Commands::Lambdaman => Ok("get lambdaman".to_string()),
        Commands::LambdamanGet { problem_id } => Ok(format!("get lambdaman{}", problem_id)),
        Commands::LambdamanSubmit {
            problem_id,
            filepath,
        } => {
            let contents = read_content(&filepath)?;
            Ok(format!("solve lambdaman{} {}", problem_id, contents))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let auth_token = "5b4a264f-5e00-433c-ac1b-1f9a8b30f161".to_string();
    let client = ICFPCClient::new(auth_token);

    let message = select_content(args.command.clone())?;
    let encoded_message = encode(message)?;

    let response_message = client.post_message(encoded_message).await?;
    let decoded_message = match args.command {
        // 巨大な文字列を解釈するための問題なので、decode しちゃダメ
        Commands::EfficiencyGet { .. } => response_message,
        _ => decode(response_message)?,
    };
    println!("{}", decoded_message);

    Ok(())
}
