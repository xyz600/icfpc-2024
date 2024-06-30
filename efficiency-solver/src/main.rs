use clap::Parser;
use core::parser::ast::{self, Node, NodeFactory};
use core::parser::tokenizer;
use std::collections::VecDeque;
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

    let token_list = tokenizer::tokenize(contents)?;
    let mut queue = VecDeque::from_iter(token_list);
    let mut node_factory = NodeFactory::new();
    let mut node = ast::parse(&mut queue, &mut node_factory)?;
    println!("{}", node.to_dot_string());

    for iter in 0..10_000_000 {
        let new_node = ast::evaluate_once(node.clone(), &mut node_factory)?;
        if new_node == node {
            break;
        }
        node = new_node;

        eprintln!("iter = {}, node_size: {}", iter, node.len());
    }

    Ok(())
}
