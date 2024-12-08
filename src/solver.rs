use std::collections::HashMap;
use std::io;

pub struct Database<'a> {
    pub data: &'a str,
}

impl Database<'_> {
    pub fn new(data: &str) -> Result<Database, &'static str> {
        Ok(Database {
            data
        })
    }

    pub fn get_matching_words(&self, input_word: &str) -> Result<Vec<String>, io::Error> {
        let input_word_length: usize = input_word.len();
        let input_word_sig: HashMap<char, u8> = self.get_word_signature(&input_word.to_lowercase());

        let mut matching_words: Vec<String> = Vec::new();

        for line in self.data.lines() {
            if line.is_empty() { continue }
            if line.len() != input_word_length { continue }

            let row_sig: HashMap<char, u8> = self.get_word_signature(&line.to_lowercase());
            if row_sig != input_word_sig { continue }

            matching_words.push(line.to_string());
        }

        Ok(matching_words)
    }

    fn get_word_signature(&self, word: &str) -> HashMap<char, u8> {
        // creates a signature of a word with characters in word and their count

        let mut map: HashMap<char, u8> = HashMap::with_capacity(word.len());

        for ch in word.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        map
    }
}
