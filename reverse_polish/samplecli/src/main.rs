// v1 builderパターン

// use clap::{App, Arg};

// fn main() {
//     let formula_file = "formula_file";

//     let matches = App::new("My RPN program")
//         .version("1.0.0")
//         .author("Masakazu sano")
//         .about("Super awesome RPN calc!")
//         .arg(
//             Arg::new(formula_file)
//                 .about("Formula written in RPN")
//                 .value_name("FILE")
//                 .index(1)
//                 .required(false)
//         )
//         .arg(
//             Arg::new("verbose")
//                 .about("Sets the level of verbosity")
//                 .short('v') // shortはcharなのでシングルクウォートで囲む
//                 .long("verbose")
//                 .required(false)
//         )
//         .get_matches();

//     match matches.value_of(formula_file) {
//         Some(file) => println!("File specified: {}", file),
//         None => println!("No file specified."),
//     }
//     let verbose = matches.is_present("verbose");
//     println!("Is verbosity specified?: {}", verbose);
// }


// v2 deriveを利用したパターン

// - clapは `3.0.0-rc.9` を入れないとダメらしい
// 	- 実践Rustプログラミング入門でうまく動かなかったとこメモ https://zenn.dev/esaka/scraps/2ced052b6d5903
// 	- 更に、cargo.tomlにfeaturesを足す必要がある `clap = { version = "3.0.0-rc.9", features = ["derive"] }`
// 		- フューチャーフラグとは https://qiita.com/osanshouo/items/43271813b5d62e89d598

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {

    let opts = Opts::parse();

    // 指定ファイルが存在する場合
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // 指定ファイルが存在しない場合
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

// ファイル、標準出力どちらも引数として渡せるようにしている（<R: BufRead>）
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        // 先頭から文字を評価していきたいので、rev()してる
        // collect::<Vec<_>>
        // コレクション変換、Vec内部の型は推論するよう「_」指定をしている
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            // 数字の場合
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else { // 演算子の場合
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            // `-v` オプションが指定されている場合、この時点でのトークンとスタックの状態を出力する
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax");
        }
    }
}

// cfgは特別なアトリビュート形式。cargo build / run時などは無視される
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // このアトリビュート直下のfnは、testコマンド実行時にテストコードとして処理される
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ."); // 使用できない演算子を使用し、意図的にpanicさせる
    }
}