use std::{fs::File, io::Write};
use std::io::copy;
use std::path::Path;
use anyhow::{anyhow, Context, Result};

pub struct DatabaseDownloader {
    pub url: String
}

impl DatabaseDownloader {
    pub fn download_db(&self, save_path: &Path) -> Result<()> {
        let mut response = ureq::get(&self.url)
            .call()
            .context("failed to connect to database URL")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "error downloading database from the interweb, server returned status {}", 
                response.status(), 
            ));
        }

        let mut response_reader = response.body_mut().as_reader();

        let mut file: File = File::create(save_path)
            .context("cannot create database file")?;

        copy(&mut response_reader, &mut file)
            .context("error saving downloaded database file")?;
        
        file.flush().context("error flushing db data to disk")?;

        Ok(())
    }
}
