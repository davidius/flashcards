mod types {
    pub mod config_file;
    pub mod flashcard_file;
}
mod assessment;
mod config;
mod logger;
mod run_flashcards;
mod yaml_parse;

use std::fs;

use clap::{Parser, Subcommand};
use config::{
    add_flashcard_file, get_list_of_flashcard_files, get_matching_flashcard_file_location,
    list_flashcard_files,
};
use inquire::{InquireError, Select};
use types::flashcard_file::QuestionFile;
use yaml_parse::parse_flashcards_yaml;

use crate::run_flashcards::run_flashcards;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddFlashcardFile { filename: Option<String> },
    Run { questionset_name: Option<String> },
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::AddFlashcardFile { filename } => match filename {
            Some(filename) => add_flashcard_file(filename.clone()),
            None => println!("No filename specified"),
        },
        Commands::Run { questionset_name } => match questionset_name {
            Some(questionset_name) => {
                println!("Running flashcards: {}", questionset_name);

                let flashcard_file_location =
                    get_matching_flashcard_file_location(questionset_name.to_string());

                match flashcard_file_location {
                    Some(flashcard_file_location) => {
                        let raw_flashcards_yaml_result =
                            fs::read_to_string(&flashcard_file_location);

                        let raw_flashcards_yaml = match raw_flashcards_yaml_result {
                            Ok(raw_flashcards_yaml) => raw_flashcards_yaml,
                            Err(e) => {
                                println!("Error reading flashcards file: {}", e);
                                println!("Attempted to read file: {}", flashcard_file_location);
                                return;
                            }
                        };

                        let flashcards_file: QuestionFile =
                            parse_flashcards_yaml(raw_flashcards_yaml);
                        run_flashcards(flashcards_file);
                    }
                    None => {}
                }
            }
            None => {
                println!("No flashcards file specified");
                let question_sets = get_list_of_flashcard_files();
                let question_set_names: Vec<&str> =
                    question_sets.iter().map(|s| s.name.as_str()).collect();

                let ans: Result<&str, InquireError> =
                    Select::new("Which flashcards to do?", question_set_names).prompt();

                match ans {
                    Ok(choice) => {
                        let flashcard_file_location =
                            get_matching_flashcard_file_location(choice.to_string());

                        match flashcard_file_location {
                            Some(flashcard_file_location) => {
                                let raw_flashcards_yaml_result =
                                    fs::read_to_string(&flashcard_file_location);

                                let raw_flashcards_yaml = match raw_flashcards_yaml_result {
                                    Ok(raw_flashcards_yaml) => raw_flashcards_yaml,
                                    Err(e) => {
                                        println!("Error reading flashcards file: {}", e);
                                        println!(
                                            "Attempted to read file: {}",
                                            flashcard_file_location
                                        );
                                        return;
                                    }
                                };

                                let flashcards_file: QuestionFile =
                                    parse_flashcards_yaml(raw_flashcards_yaml);
                                run_flashcards(flashcards_file);
                            }
                            None => {}
                        }
                    }
                    Err(_) => println!("There was an error, please try again"),
                }
            }
        },
        Commands::List => list_flashcard_files(),
    }
}
