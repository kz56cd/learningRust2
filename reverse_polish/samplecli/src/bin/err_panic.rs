// v1: エラーハンドリングなし
// fn get_int_from_file() -> i32 {
//     let path = "number.txt";

//     let num_str = std::fs::read_to_string(path)
//         .expect("failed to open the file.");

//     let ret = num_str
//         .trim()
//         .parse::<i32>()
//         .expect("failed to parse string to number.");
//     ret * 2
// }

// fn main() {
//     println!("{}", get_int_from_file());
// }

// v2: エラーハンドリングあり（Result型で対応）

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

use std::fmt;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
         }
    }
}

impl From<std::io::Error> for MyError {
    fn from(cause: std::io::Error) -> Self {
        Self::Io(cause)
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)
        // .map_err(|e| MyError::Io(e))?;
        .map_err(MyError::from)?; // fromトレイトを使うとシンプルに記述できたりする

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O Error: {}", cause),
            MyError::Num(cause) => println!("Parse Error: {}", cause),
        },
    }
}