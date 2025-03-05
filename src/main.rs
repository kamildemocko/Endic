mod database_utils;
mod config;
mod search;
mod printer;
mod args;

use std::path::PathBuf;
use std::fs;
use anyhow::{anyhow, Ok, Result};

use crate::search::SearchDb;

fn main() -> Result<()>{
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    let (match_word, query) = args::get_args();

    let mut db_path: PathBuf = PathBuf::new();
    config::get_db_filepath(&mut db_path);

    verify_db_file(&db_path)?;

    let mut searcher: SearchDb = SearchDb::new(&db_path)?;
    let res = searcher.search_db(query, match_word)?;
    printer::print_results(&res);

    Ok(())
}

fn verify_db_file(db_path: &PathBuf) -> Result<()> {
    let db_file_exists: bool = db_path.exists();
    if !db_file_exists {
        println!("downloading database file from the interweb");

        let db_path_parent = db_path.parent()
            .ok_or_else(|| anyhow!("path {:?} has no parent", db_path))?;
        fs::create_dir_all(db_path_parent)?;

        database_utils::GetDB{url: config::DB_DOWNLOAD_URL.to_string()}
            .download_db(db_path.as_path())?;
    }

    Ok(())
}
