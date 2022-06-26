use std::{env, io};
use std::io::BufRead;

use pluralizer::pluralize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn usage() {
    eprintln!("\nusage: pluralize-cli [-p, --plural] [-s, --singular] [-v, --version] [WORD]\n");
}

fn version() {
    println!("{} {}", NAME, VERSION);
}

fn pluralize_word(word: &str, count: isize) {
    let pluralized_word = pluralize(word, count, false);
    println!("{}", pluralized_word);
}

fn pluralize_input(count: isize) {
    let stdin = io::stdin();
    for word in stdin.lock().lines() {
        pluralize_word(word.unwrap().as_str(), count);
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 2 indicates plural, 1 indicates singular
    let count = 2;

    pluralizer::initialize();

    let arg_len = args.len();
    match arg_len {
        1 => pluralize_input(count),
        2 => match args[1].as_str() {
            "-v" => version(),
            "-s" => pluralize_input(1),
            "-p" => pluralize_input(count),
            _ => {
                usage();
                return Err("Invalid flag".into());
            }
        },
        3 => {
            let word = args[2].as_str();
            match args[1].as_str() {
                "-s" => pluralize_word(word, 1),
                "-p" => pluralize_word(word, count),
                _ => {
                    usage();
                    return Err("Invalid flag".into());
                }
            }
        }
        _ => {
            usage();
            return Err("Too many arguments".into());
        }
    }
    Ok(())
}
