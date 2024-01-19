use std::fs::File;
use std::io::{copy, Cursor};
use std::path::Path;
use bytes::Bytes;
use reqwest::blocking::Response;

pub struct GetDB<'a> {
    pub url: &'a str,
}

impl GetDB<'_> {
    pub fn download_db(&self, save_path: &Path) {
        let response: Response = reqwest::blocking::get(self.url).expect("cannot make request that downloads db");
        if response.status() == 404 {
            panic!("{}", "error downloading database from the interweb")
        }

        let db_download_response = response.bytes().unwrap();

        let mut content: Cursor<Bytes> = Cursor::new(db_download_response);
        let mut file: File = File::create(save_path).expect("cannot create database file");

        copy(&mut content, &mut file).expect("error saving downloaded database file");
    }
}
