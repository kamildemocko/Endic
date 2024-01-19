use std::env;

pub fn get_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        eprintln!("usage: script QUERY");
        std::process::exit(1);
    }

    args.join(" ")
}
