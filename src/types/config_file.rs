use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ConfigFile {
    pub flashcard_files:Vec<ConfigFlashcardFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ConfigFlashcardFile {
    pub location: String,
    pub name: String,
}