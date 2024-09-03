use chrono::{Datelike, Days, Utc};
use dateparser::parse_with_timezone;
use inquire::{DateSelect, InquireError, Select, Text};

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

        match flashcard.question_type {
            QuestionType::SingleAnswer => {
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
                    Err(_) => {
                        println!("An error happened when asking for your name, try again later.")
                    }
                }
            }
            QuestionType::MultipleChoice => {
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
            QuestionType::Date => {
                let answer_date_str = flashcard.answer.clone();
                let answer_date = parse_with_timezone(&answer_date_str, &Utc);
                match answer_date {
                    Ok(answer_date) => {
                        let naive_answer_date = chrono::NaiveDate::from_ymd_opt(
                            answer_date.year(),
                            answer_date.month(),
                            answer_date.day(),
                        )
                        .unwrap();

                        let (min_date, max_date) = get_min_and_max_dates(answer_date);

                        let user_date = DateSelect::new(&flashcard.question.as_str())
                            .with_default(naive_answer_date)
                            .with_min_date(min_date)
                            .with_max_date(max_date)
                            .with_week_start(chrono::Weekday::Mon)
                            .prompt();

                        match user_date {
                            Ok(_) => {
                                if user_date.as_ref().unwrap().year() == answer_date.year()
                                    && user_date.as_ref().unwrap().month() == answer_date.month()
                                    && user_date.as_ref().unwrap().day() == answer_date.day()
                                {
                                    number_of_flashcards_correct += 1;
                                    print_correct();
                                } else {
                                    print_incorrect_with_answer(&flashcard.answer);
                                }
                            }
                            Err(_) => println!("There was an error in the system."),
                        }
                    }
                    _error => {
                        println!("Could not parse date");
                    }
                }
            }
        }
    }
    let proportion_correct = number_of_flashcards_correct as f32 / number_of_flashcards as f32;
    print_correct_of_total(number_of_flashcards_correct, number_of_flashcards);
    print_message_based_on_score(proportion_correct);
}

// TODO: this function needs work...
fn get_min_and_max_dates(
    answer_date: chrono::DateTime<Utc>,
) -> (chrono::NaiveDate, chrono::NaiveDate) {
    let min_date =
        chrono::NaiveDate::from_ymd_opt(answer_date.year(), answer_date.month(), answer_date.day())
            .unwrap()
            .checked_sub_days(Days::new(30))
            .unwrap();
    let max_date =
        chrono::NaiveDate::from_ymd_opt(answer_date.year(), answer_date.month(), answer_date.day())
            .unwrap()
            .checked_add_days(Days::new(30))
            .unwrap();

    (min_date, max_date)
}
