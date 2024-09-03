use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct QuestionFile {
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub question_type: QuestionType,
    /// A list of wrong answers to display when using multiple choice
    pub alt_answers: Option<Vec<String>>,
    pub use_randomised_alt_answers: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum QuestionType {
    SingleAnswer,
    MultipleChoice,
    Date,
}
