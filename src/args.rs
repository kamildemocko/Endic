use std::env;

pub fn get_args() -> (bool, String) {
    let args = env::args().skip(1);
    let mut flag_match_word = false;
    let mut words: Vec<String> = Vec::new();

    if args.len() == 0 {
        eprintln!("usage: script QUERY");
        std::process::exit(1);
    }

    for word in args {
        if word == ("-m") || word == "--match" {
            flag_match_word = true;
            continue
        }

        words.push(word);
    }

    (flag_match_word, words.join(" "))
}
