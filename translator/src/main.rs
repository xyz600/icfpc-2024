use clap::Parser;
use core::parser;

use core::parser::ast::Node;
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

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    // ファイルの内容を読み込む
    match fs::read_to_string(&args.file) {
        Ok(contents) => {
            let result_node = parser::parse(contents)?;
            match result_node {
                Node::String(_, s) => {
                    for c in s.iter() {
                        print!("{}", c);
                    }
                    println!();
                }
                _ => {
                    println!("cannot reduce to string: {:?}", result_node);
                }
            }
            Ok(())
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            Err(error.into())
        }
    }
}
