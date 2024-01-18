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
        let response: Response = reqwest::blocking::get(self.url).unwrap();
        if response.status() == 404 {
            panic!("{}", "error downloading database from the interweb")
        }

        let db_download_response = response.bytes().unwrap();

        let mut content: Cursor<Bytes> = Cursor::new(db_download_response);
        let mut file: File = File::create(save_path).unwrap();

        let res_save_file = copy(&mut content, &mut file);
        match res_save_file {
            Ok(_) => (),
            Err(e) => panic!("Error saving file {e}"),
        }

    }
}
