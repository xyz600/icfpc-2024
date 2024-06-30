use clap::Parser;
use core::parser::ast::parse;
use std::fs;
use std::path::PathBuf;

/// このプログラムはコマンドライン引数からファイルパスを受け取り、その内容を出力します。
#[derive(Parser, Debug, Clone)]
#[command(name = "file_reader")]
#[command(about = "A simple file reader")]
struct Args {
    #[arg(short, long)]
    filepath: PathBuf,
}

fn read_content(path: &PathBuf) -> Result<String, anyhow::Error> {
    fs::read_to_string(path).map_err(|e| e.into())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let contents = read_content(&args.filepath)?;
    let node = parse(contents)?;

    println!("{:?}", node);

    Ok(())
}
