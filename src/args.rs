use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_args() -> (bool, String) {
    let args = env::args().skip(1);
    let mut flag_match_word = false;
    let mut words: Vec<String> = Vec::new();

    if args.len() == 0 {
        println!("Usage: ");
        println!("endic [--match | --version] QUERY");
        println!(" » QUERY = Word to search for");
        println!(" » --match   | -m = Whole word search");
        println!(" » --version | -v = Shows app version");

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
