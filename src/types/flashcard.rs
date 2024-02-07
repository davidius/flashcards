use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FlashcardFile {
  pub flashcards: Vec<Flashcard>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Flashcard {
  pub question: String,
  pub answer: String,
  // pub tags: Vec<String>,
}