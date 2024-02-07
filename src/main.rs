mod types {
    pub mod flashcard;
}
mod yaml_parse;
mod assessment;

use ansi_term::{Colour, Style};
use assessment::print_message_based_on_score;
use clap::Parser;
use types::flashcard::FlashcardFile;
use yaml_parse::parse_flashcards_yaml;
use std::fs;

use crate::assessment::print_correct_of_total;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read flashcards from
    flashcards_file: Option<String>,
}

const FAILURE_COLOUR: Colour = Colour::Red;
const SUCCESS_COLOUR: Colour = Colour::Cyan;

fn main() {
    let args = Args::parse();

    // let blerg = args.flashcards_file.as_deref();

    let flashcards_file = match args.flashcards_file {
        Some(flashcards_file) => flashcards_file,
        None => {
            println!("No flashcards file specified");
            return;
        }
    };

    let raw_flashcards_yaml_result = fs::read_to_string(&flashcards_file);
    
    let raw_flashcards_yaml = match raw_flashcards_yaml_result {
        Ok(raw_flashcards_yaml) => raw_flashcards_yaml,
        Err(e) => {
            println!("Error reading flashcards file: {}", e);
            println!("Attempted to read file: {}", flashcards_file);
            return;
        }
    };
    let flashcards_file: FlashcardFile = parse_flashcards_yaml(raw_flashcards_yaml);

    let number_of_flashcards = flashcards_file.flashcards.len();
    let mut number_of_flashcards_correct = 0;

    for flashcard in flashcards_file.flashcards {
        println!("====================");
        println!("{}", flashcard.question);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == flashcard.answer {
            number_of_flashcards_correct += 1;
            println!(
                "{}",
                Style::new().bold().fg(SUCCESS_COLOUR).paint("Correct!"),
            );
        } else {
            println!(
                "{}",
                Style::new().bold().fg(FAILURE_COLOUR).paint("Incorrect :("),
            );
            println!("Correct answer: {}", flashcard.answer);
        }
    }
    let proportion_correct = number_of_flashcards_correct as f32 / number_of_flashcards as f32;
    print_correct_of_total(number_of_flashcards_correct, number_of_flashcards);
    print_message_based_on_score(proportion_correct);
}
