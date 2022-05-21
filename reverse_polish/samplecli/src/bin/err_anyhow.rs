// -----------------------------------
// anyhowクレートを使ったエラー処理
// -----------------------------------

// use anyhow::{Context, Result};
use anyhow::{bail, ensure, Context, Result};

// anyhowの場合：返り値はResultの正常型だけでよい
// このResultはanyhow独自の型（ = useで宣言している）
fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?; // anyhowの場合：map_errの代わりにwith_context

    if num_str.len() >= 10 {
        // 10桁以上であればエラー
        bail!("it may be too large number");
    }

    // 1桁目が1以外であればエラー
    ensure!(num_str.starts_with("1"), "first digit is not 1");

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .context("failed to parse string") // anyhowの場合：map_errの代わりにcontext
}

fn main() {
    match get_int_from_file() {
        // Ok(x) => println!("{}", x),
        // Err(e) => println!("{}", e),
        Ok(x) => println!("{:#?}", x), // {:#?} はpretty printする
        Err(e) => println!("{:#?}", e),
    }
}