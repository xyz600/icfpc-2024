use clap::Parser;

use core::parser::ast::{parse, NodeType};
use core::parser::icfpstring::ICFPString;
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

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let contents = get_content(&args.file)?;

    if args.encode {
        let s = ICFPString::from_encoded_str(&contents.as_str())?;
        let encoded = s.to_string()?.into_iter().collect::<String>();
        println!("S{}", encoded);
        Ok(())
    } else {
        let result_node = parse(contents)?;
        match result_node.node_type {
            NodeType::String(s) => {
                for c in s.iter() {
                    print!("{}", c);
                }
                println!();
                Ok(())
            }
            _ => {
                println!("cannot reduce to string: {:?}", result_node);
                println!("result_node.to_dot_string()");
                Err(anyhow::anyhow!("cannot reduce to string"))
            }
        }
    }
}
