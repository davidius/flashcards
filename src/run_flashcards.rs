use inquire::{InquireError, Select, Text};

use crate::{
    assessment::{print_correct_of_total, print_message_based_on_score},
    logger::{print_correct, print_incorrect_with_answer},
    types::flashcard_file::{QuestionFile, QuestionType},
};

pub fn run_flashcards(question_file: QuestionFile) {
    let number_of_flashcards = question_file.questions.len();

    let mut number_of_flashcards_correct = 0;

    for flashcard in question_file.questions {
        println!("====================");

        if flashcard.question_type == QuestionType::SingleAnswer {
            let input = Text::new(flashcard.question.as_str()).prompt();

            match input {
                Ok(answer) => {
                    if answer.trim() == flashcard.answer {
                        number_of_flashcards_correct += 1;
                        print_correct();
                    } else {
                        print_incorrect_with_answer(&flashcard.answer);
                    }
                }
                Err(_) => println!("An error happened when asking for your name, try again later."),
            }
        } else if flashcard.question_type == QuestionType::MultipleChoice {
            let alt_answers = flashcard.alt_answers;
            match alt_answers {
                Some(alt_answers) => {
                    let alt_answers_vec: Vec<&str> = alt_answers
                        .iter()
                        .map(|s| s.as_str())
                        .chain(vec![flashcard.answer.as_str()])
                        .collect();

                    let ans: Result<&str, InquireError> =
                        Select::new(&flashcard.question, alt_answers_vec).prompt();

                    match ans {
                        Ok(choice) => {
                            if choice == flashcard.answer {
                                number_of_flashcards_correct += 1;
                                print_correct();
                            } else {
                                print_incorrect_with_answer(&flashcard.answer);
                            }
                        }
                        Err(_) => println!("There was an error, please try again"),
                    }
                }
                None => {}
            }
        }
    }
    let proportion_correct = number_of_flashcards_correct as f32 / number_of_flashcards as f32;
    print_correct_of_total(number_of_flashcards_correct, number_of_flashcards);
    print_message_based_on_score(proportion_correct);
}
