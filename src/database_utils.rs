use std::fs::File;
use std::io::{copy, Cursor};
use std::path::Path;
use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use reqwest::blocking::Response;

pub struct GetDB {
    pub url: String
}

impl GetDB {
    pub fn download_db(&self, save_path: &Path) -> Result<()> {
        let response: Response = reqwest::blocking::get(&self.url)
            .context("cannot make request that downloads db")?;
        if response.status() == 404 {
            return Err(anyhow!("error downloading database from the interweb"));
        }

        let db_download_response = response.bytes()
            .context("cannot get bytes from response when downloading DB")?;

        let mut content: Cursor<Bytes> = Cursor::new(db_download_response);
        let mut file: File = File::create(save_path).context("cannot create database file")?;

        copy(&mut content, &mut file).context("error saving downloaded database file")?;

        Ok(())
    }
}
