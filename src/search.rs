use std::fs::File;
use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};
use csv::Reader;
use indexmap::IndexMap;

pub struct SearchDb {
    reader: Reader<File>
}

#[derive(Debug)]
pub struct SearchItem {
    pub name: String,
    pub word_type: String,
    pub meanings: Vec<String>,
}

impl SearchDb {
    pub fn new(db_path: &PathBuf) -> Result<SearchDb> {
        Ok(SearchDb {
            reader: csv::Reader::from_path(db_path.as_path())
                .context("cannot open database")?
        })
    }

    pub fn search_db(&mut self, query: String, match_word: bool) -> Result<IndexMap<String, SearchItem>> {
        let mut result_items: IndexMap<String, SearchItem> = IndexMap::new();

        for item in self.reader.records() {
            let item_unwrapped = item?;

            let word_type: &str = item_unwrapped.get(1)
                .ok_or_else(|| anyhow!("missing word type in database record"))?;
            let name = item_unwrapped.get(0)
                .ok_or_else(|| anyhow!("missing word type in database record"))?;
            let meanings_group = item_unwrapped.get(2)
                .ok_or_else(|| anyhow!("missing meanings in database record"))?;
            let meanings: Vec<String> = meanings_group
                .split(";")
                .map(|m| m.trim().to_string())
                .collect();

            if match_word {
                if name.to_lowercase() == query.to_lowercase() {
                    result_items
                        .entry(name.to_string())
                        .and_modify(|val| val.meanings.extend(meanings.clone()))
                        .or_insert(
                        SearchDb::prepare_search_item(name, word_type, meanings.clone())
                        );
                }
            } else {
                if name.to_lowercase().starts_with(&query.to_lowercase()) {
                    result_items
                        .entry(name.to_string())
                        .and_modify(|val| val.meanings.extend(meanings.clone()))
                        .or_insert(
                        SearchDb::prepare_search_item(name, word_type, meanings.clone())
                        );
                }
            }
        }

        Ok(result_items)
    }

    fn prepare_search_item(name: &str, word_type: &str, meanings: Vec<String>) -> SearchItem {
        SearchItem {
            name: String::from(name),
            word_type: String::from(word_type),
            meanings,
        }
    }
}