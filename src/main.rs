mod database_utils;
mod config;
mod search;
mod printer;

use std::path::PathBuf;
use std::fs;
use clap::Parser;
use crate::search::SearchDb;

#[derive(Parser, Debug)]
struct Args {
    query: String,
}

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    let args = Args::parse();

    let mut db_path: PathBuf = PathBuf::new();
    config::get_db_filepath(&mut db_path);

    verify_db_file(&db_path);

    let query: String = args.query;

    let mut searcher: SearchDb = SearchDb::new(&db_path);
    let res = searcher.search_db(query);
    printer::print_results(res);
}

fn verify_db_file(db_path: &PathBuf) {
    let db_file_exists: bool = db_path.exists();
    if !db_file_exists {
        println!("downloading database file from the interweb");

        fs::create_dir_all(db_path.parent().unwrap()).unwrap();

        database_utils::GetDB{
            url: config::DB_DOWNLOAD_URL,
        }.download_db(db_path.as_path());
    }
}
