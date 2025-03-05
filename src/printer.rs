use crate::search::SearchItem;
use indexmap::IndexMap;

use colored::Colorize;

pub fn print_results(items: &IndexMap<String, SearchItem>) {
    if items.len() == 0 {
        println!("No result!");
        return
    }

    for (_, value) in items.iter() {
        let word_type = if value.word_type.is_empty() {
            "n/a".to_string()
        } else {
            value.word_type.clone()
        };

        println!("{} ({})",
                 value.name.purple(),
                 word_type.bright_black()
                );

        for m in &value.meanings {
            println!("  »  {}", m.yellow());
        }
    }
}
