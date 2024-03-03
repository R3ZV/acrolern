mod cli;

use std::{
    collections::HashSet,
    error::Error,
    fs::{read_to_string, File},
    io::{self, stdout, Write},
};

use colored::*;
use serde::{Deserialize, Serialize};

use clap::Parser;
use cli::{Cli, Command};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    acronym: String,
    meaning: String,
    score: usize,
    description: String,
    tags: HashSet<String>,
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

fn game_loop(
    questions: &mut Vec<Question>,
    desc: Option<bool>,
    tags: Option<Vec<String>>,
    score: Option<usize>,
) {
    clear_terminal();
    for question in questions {
        if let Some(ref tags) = tags {
            let mut has_tag = false;
            for tag in &question.tags {
                if tags.contains(&tag) {
                    has_tag = true;
                    break;
                }
            }
            if !has_tag {
                continue;
            }
        }

        if let Some(score) = score {
            if question.score > score {
                continue;
            }
        }

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
        if let Some(desc) = desc {
            if desc {
                println!("{}\n", question.description);
            }
        }

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
}

fn get_tags(questions: &Vec<Question>) -> HashSet<String> {
    let mut tags: HashSet<String> = HashSet::new();
    for question in questions {
        for tag in &question.tags {
            tags.insert(tag.to_string());
        }
    }
    return tags;
}

fn main() {
    let cli = Cli::parse();

    let file = String::from("/home/r3zv/dev/acrolern/src/acronyms.json");
    let mut questions = parse_questions(&file).expect("Couldn't parse the acronyms");

    dbg!(&cli.tags);
    match cli.command {
        Command::Play => {
            questions.sort_by(|a, b| a.score.cmp(&b.score));

            game_loop(&mut questions, cli.desc, cli.tags, cli.upto);

            println!("Saving session...");
            match save_questions(&questions, file) {
                Ok(_) => println!("Session data saved!"),
                Err(_) => println!("Couldn't save session data"),
            }
        }
        Command::Tags => {
            let tags = get_tags(&questions);
            println!("{}", "Available tags:");
            for tag in tags {
                print!("{} ", tag.green());
            }
            println!();
        }
    }
}
