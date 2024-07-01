use clap::Parser;
use num_bigint::BigInt;

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
    filepath: PathBuf,
}

fn get_content(path: &PathBuf) -> Result<String, anyhow::Error> {
    fs::read_to_string(path).map_err(|e| e.into())
}

// 生コマンドが入った文字列を返す
// この文字列を評価すると、Integer(v) が得られる
fn compress(v: BigInt) -> Result<String, anyhow::Error> {
    // I"..."
    let raw_string = ICFPString::from_int(v.clone())
        .to_string()?
        .into_iter()
        .collect::<String>();
    let bypass_str = format!("I{}", raw_string);

    // 94進数で1桁で書けるなら、流石にこっちの方が短そう
    if v < BigInt::from(94) {
        Ok(bypass_str)
    } else {
        let q = v.sqrt();
        let r = v - q.clone() * q.clone();

        // q * q := "B$ L# B* v# v# I(q)"
        // q * q + r := "B+ I(r) B$ L# B* v# v# I(q)"
        // int2str(q * q + r) := "U$ B+ I(r) B$ L# B* v# v# I(q)"
        // cost := len(q) + len(r) +

        let f_q = compress(q)?;
        let f_r = compress(r)?;

        let compressed_string = format!("B+ {} B$ L# B* v# v# {}", f_r, f_q);
        if bypass_str.len() < compressed_string.len() {
            Ok(bypass_str)
        } else {
            Ok(compressed_string)
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let contents = get_content(&args.filepath)?;

    // 生文字列を読み込む

    // 以下の2つの中から、短い選択肢を選ぶ
    // 1. 即値命令(`S...`)
    // 2. 以下のような再帰的な計算
    //     1. 表現したい文字列 S を数値化して v に
    //     2. q = floor(sqrt(v))
    //     3. r = v - q * q
    //     4. f(r) と f(v) を計算
    //     5. s = int2str(f(r)) + (λ.x x*x) f(v)
    //     6. int2str(s)

    let s = ICFPString::from_encoded_str(&contents.as_str())?;
    let v = s.to_int();
    let encoded = compress(v)?;
    println!("U$ {}", encoded);

    Ok(())
}
