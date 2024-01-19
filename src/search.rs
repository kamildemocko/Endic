use std::fs::File;
use std::path::PathBuf;
use csv::Reader;

pub struct SearchDb {
    reader: Reader<File>
}

pub struct SearchItem {
    pub name: String,
    pub word_type: String,
    pub meaning: String,
}

impl SearchDb {
    pub fn new(db_path: &PathBuf) -> SearchDb {
        SearchDb {reader: csv::Reader::from_path(db_path.as_path())
            .expect("cannot open database")}
    }

    pub fn search_db(&mut self, query: String) -> Vec<SearchItem> {
        let mut result_items: Vec<SearchItem> = vec!();

        for item in self.reader.records() {
            let item_unwrapped = item.unwrap();

            let word_type: &str = item_unwrapped.get(1).unwrap();
            let name = item_unwrapped.get(0).unwrap();
            let meaning = item_unwrapped.get(2).unwrap();

            if name.to_lowercase() == query.to_lowercase() {
                let result_item: SearchItem = SearchItem {
                    name: String::from(name),
                    word_type: String::from(word_type),
                    meaning: String::from(meaning),
                };
                result_items.push(result_item);
            }
        }

        return result_items;
    }
}
