mod database_utils;
mod config;
mod search;
mod printer;

use std::path::Path;
use std::fs;
use clap::Parser;
use crate::search::{SearchDb};

#[derive(Parser, Debug)]
struct Args {
    query: String,
}

fn main() {
    let args = Args::parse();
    verify_db_file();

    let query: String = args.query;

    let mut searcher: SearchDb = search::SearchDb::new();
    let res = searcher.search_db(query);
    printer::print_results(res);
}

fn verify_db_file() {
    let db_path = Path::new(config::DB_FILEPATH);

    let db_file_exists: bool = Path::new(db_path).exists();
    if !db_file_exists {
        println!("downloading database file from the interweb");

        fs::create_dir_all(db_path.parent().unwrap()).unwrap();

        database_utils::GetDB{
            url: config::DB_DOWNLOAD_URL,
        }.download_db(db_path);
    }
}
