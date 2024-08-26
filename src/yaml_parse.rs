use crate::types::{config_file::ConfigFile, flashcard_file::*};

/// Parses the flashcards yaml file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty FlashcardFile object.
pub fn parse_flashcards_yaml(raw_flashcards_yaml: String) -> QuestionFile {
    let flashcard_file_result = serde_yaml::from_str(&raw_flashcards_yaml);
    match flashcard_file_result {
        Ok(flashcard_file) => flashcard_file,
        Err(_) => QuestionFile { questions: vec![] },
    }
}

pub fn parse_config_yaml(raw_config_yaml: String) -> ConfigFile {
    let config_file_result = serde_yaml::from_str(&raw_config_yaml);
    match config_file_result {
        Ok(config_file) => config_file,
        Err(_) => ConfigFile {
            flashcard_files: vec![],
        },
    }
}
