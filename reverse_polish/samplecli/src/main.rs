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

// v2
use clap::Clap;

// TODO: ビルドエラー解消する
#[derive(Clap, Debug)]


// #[clap(
//     name = ("My RPN program"),
//     version = "1.0.0",
//     author = "Masakazu sano",
//     about = "Super awesome RPN calc!"
// )]

// struct Opts {
//     // Set the level of verbosity
//     #[clap(short, long)]
//     verbose: bool,

//     #[clap(name = "FILE")]
//     formula_file: Option<String>,
// }

// fn main() {
//     let opts = Opts::parse();

//     match opts.formula_file {
//         Some(file) => println!("File specified: {}", file),
//         None => println!("No file specified."),
//     }
//     println!("Is verbosity specified?: {}", opts.verbose);
// }