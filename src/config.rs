use std::path::PathBuf;

pub const DB_DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/benjihillard/English-Dictionary-Database/main/english%20Dictionary.csv";

pub fn get_db_filepath(path: &mut PathBuf) {
    let temp: PathBuf = std::env::temp_dir();
    path.push(temp);
    path.push("endic_with_dnld");
    path.push("db.csv");
}