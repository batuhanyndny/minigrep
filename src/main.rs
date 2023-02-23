mod lib;

use lib::{arg_parser::MinigrepArgs, minigrep};
use std::{env, process};

fn main() {
    let args = MinigrepArgs::parse(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let content = minigrep::open_file(&args.haystack).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let result = minigrep::search(&args.needle, &content, args.ignore_case);
    match result {
        Some(v) => {
            for line in v.iter() {
                println!("{}", line);
            }
        }
        None => println!("Not found"),
    }
}
