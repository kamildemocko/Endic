use crate::search::SearchItem;

use colored::Colorize;

pub fn print_results(items: Vec<SearchItem>) {
    for mut item in items {
        if item.word_type == "" {
            item.word_type = "n/a".to_string();
        }

        println!("{} ({})",
                 item.name.purple(),
                 item.word_type.bright_black());

        for m in item.meanings {
            println!("  ◼  {}", m.yellow());
        }
    }
}
