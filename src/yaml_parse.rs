use crate::types::flashcard::*;

/// Parses the flashcards yaml file and retrieves its values. Consumes any errors that can happen while parsing, and in
/// case of any error just returns an empty FlashcardFile object.
pub fn parse_flashcards_yaml(raw_flashcards_yaml: String) -> FlashcardFile {
  let flashcard_file_result = serde_yaml::from_str(&raw_flashcards_yaml);
  match flashcard_file_result {
      Ok(flashcard_file) => flashcard_file,
      Err(_) => FlashcardFile {
          flashcards: vec![],
      },
  }
}