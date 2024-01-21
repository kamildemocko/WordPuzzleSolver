pub struct Printer;

impl Printer {
    fn get_input_characters_as_title(value: String) -> String {
        let characters: Vec<String> = value
            .chars()
            .map(|ch| ch.to_string())
            .map(|ch| ch.to_uppercase())
            .collect();

        characters.join("-")
    }

    pub fn display_matching_words(input_word: String, matching_words: Vec<String>) {

        println!("Matching words for characters {}:", Printer::get_input_characters_as_title(input_word));
        for word in matching_words {
            println!(" {} {}", 187 as char, word);
        }
        println!();
    }

    pub fn display_no_matching_words(input_word: String) {
        println!("No matching words for characters {}:", Printer::get_input_characters_as_title(input_word));
    }
}