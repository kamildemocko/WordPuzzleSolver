mod solver;
mod printer;
mod arguments;

use arguments::parse_args;
use solver::Database;
use printer::Printer;

fn main() {
    let characters: String = parse_args();

    let db = include_str!("./data/word_puzzle_solver.csv");
    let database: Database = Database::new(db).expect("Failed to create database");
    let matching_words: Vec<String> = database.get_matching_words(&characters).unwrap();
    if matching_words.is_empty() {
        Printer::display_no_matching_words(characters);
    } else {
        Printer::display_matching_words(characters, matching_words);
    }
}
