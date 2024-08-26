use ansi_term::{Colour, Style};

use crate::types::config_file::ConfigFlashcardFile;

const FAILURE_COLOUR: Colour = Colour::Red;
const SUCCESS_COLOUR: Colour = Colour::Cyan;

pub fn print_correct() {
    println!(
        "{}",
        Style::new().bold().fg(SUCCESS_COLOUR).paint("Correct!"),
    );
}

pub fn print_incorrect_with_answer(answer: &str) {
    println!(
        "{}",
        Style::new().bold().fg(FAILURE_COLOUR).paint("Incorrect :("),
    );
    println!("Correct answer: {}", answer);
}

pub fn print_flashcard_file_summary(flashcard_file: ConfigFlashcardFile) {
    println!("- {}", flashcard_file.name);
    println!("  - Location: {}", flashcard_file.location);
}
