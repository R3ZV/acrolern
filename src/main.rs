use std::{
    error::Error,
    fs::{read_to_string, File},
    io::{self, stdout, Write},
};

use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    acronym: String,
    meaning: String,
    score: usize,
    description: String,
    tags: Vec<String>,
}

fn parse_questions(file: &String) -> Result<Vec<Question>, Box<dyn Error>> {
    let data = read_to_string(&file)?;
    Ok(serde_json::from_str(&data)?)
}

fn save_questions(questions: &Vec<Question>, path: String) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    Ok(serde_json::to_writer_pretty(&file, questions)?)
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let file = String::from("acronyms.json");
    let mut questions = parse_questions(&file).expect("Couldn't parse the acronyms");
    questions.sort_by(|a, b| a.score.cmp(&b.score));

    clear_terminal();
    for question in &mut questions {
        println!("What does {} stand for?", question.acronym);
        print!("> ");
        stdout().flush().expect("Error while stdout flussing");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed reading from stdin");

        if user_input.trim() == "quit" {
            println!("Session ended!");
            break;
        } else if user_input.trim() != question.meaning {
            println!(
                "{} {} stands for {}",
                "Wrong!".red(),
                question.acronym.bold(),
                question.meaning
            );
        } else {
            println!("{}", "Correct!".green());
            question.score += 1;
        }
        println!("{}\n", question.description);

        loop {
            print!("Press {} to continue", "ENTER".bold());
            stdout().flush().expect("Error while stdout flussing");

            let mut continue_quiz = String::new();
            io::stdin()
                .read_line(&mut continue_quiz)
                .expect("Failed reading from stdin");

            if continue_quiz.trim() == "" {
                clear_terminal();
                break;
            } else {
                println!("Input: {}", continue_quiz);
            }
        }
    }

    println!("Saving session...");
    match save_questions(&questions, file) {
        Ok(_) => println!("Session data saved!"),
        Err(_) => println!("Couldn't save session data"),
    }
}
