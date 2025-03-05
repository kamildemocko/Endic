use std::env;
use colored::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_args() -> (bool, String) {
    let args = env::args().skip(1);
    let mut flag_match_word = false;
    let mut words: Vec<String> = Vec::new();

    if args.len() == 0 {
        println!("{}", "Usage:".bold());
        println!("{} [{}] {}", "endic".green(), "--match | --version".blue(), "QUERY".red());
        println!(" » {} » Word to search for", "QUERY".red());
        println!(" » {}   | {} » Whole word search", "--match".blue(), "-m".blue());
        println!(" » {} | {} » Shows app version", "--version".blue(), "-v".blue());

        std::process::exit(0);
    }

    for word in args {
        if word == "-v" || word == "--version" {
            println!("endic v{VERSION}");

            std::process::exit(0);
        } else if word == "-m" || word == "--match" {
            flag_match_word = true;
            continue
        }

        words.push(word);
    }

    (flag_match_word, words.join(" "))
}
