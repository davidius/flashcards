use ansi_term::{Colour, Style};

const FAILURE_COLOUR: Colour = Colour::Red;
const SUCCESS_COLOUR: Colour = Colour::Cyan;

pub fn print_correct_of_total(number_of_correct: usize, total: usize) {
    println!("====================");
    println!(
        "{}",
        Style::new()
            .bold()
            .fg(SUCCESS_COLOUR)
            .paint(format!("{} of {} correct", number_of_correct, total)),
    );
}

pub fn print_message_based_on_score(score: f32) {
    if score == 1.0 {
        println!(
            "{}",
            Style::new()
                .bold()
                .fg(SUCCESS_COLOUR)
                .paint("You got them all right!"),
        );
    } else if score > 0.8 {
        println!(
            "{}",
            Style::new()
                .bold()
                .fg(SUCCESS_COLOUR)
                .paint("You did pretty well!"),
        );
    } else if score > 0.5 {
        println!(
            "{}",
            Style::new()
                .bold()
                .fg(FAILURE_COLOUR)
                .paint("You could do better!"),
        );
    } else {
        println!(
            "{}",
            Style::new()
                .bold()
                .fg(FAILURE_COLOUR)
                .paint("You need to study more!"),
        );
    }
}