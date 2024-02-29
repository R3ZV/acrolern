use std::{
    fs::{read_to_string, File},
    io::{self, Write},
};

#[derive(Debug)]
struct Question {
    acronym: String,
    full_name: String,
    correct_answers: usize,
}

fn read_questions(file: &String) -> Vec<Question> {
    let mut questions = Vec::new();

    if let Ok(data) = read_to_string(&file) {
        for line in data.lines() {
            let information: Vec<&str> = line.split(":").collect();
            questions.push(Question {
                acronym: information[0].to_string(),
                full_name: information[1].to_string(),
                correct_answers: information[2].parse().unwrap(),
            });
        }
    } else {
        eprintln!("Couldn't read {}", &file)
    }

    return questions;
}

fn save_questions(questions: &Vec<Question>, path: String) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    for question in questions {
        let data = format!(
            "{}:{}:{}",
            question.acronym, question.full_name, question.correct_answers
        );
        writeln!(file, "{data}")?;
    }
    Ok(())
}

fn main() {
    let file = String::from("acronyms");
    let mut questions = read_questions(&file);
    questions.sort_by(|a, b| a.correct_answers.cmp(&b.correct_answers));

    for question in &mut questions {
        println!("What does {} stand for?", question.acronym);
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed reading from stdin");

        if user_input.trim() == "quit" {
            println!("Ending session...");
            break;
        } else if user_input.trim() != question.full_name {
            println!(
                "Wrong! {} stands for {}",
                question.acronym, question.full_name
            );
        } else {
            println!("Correct!");
            question.correct_answers += 1;
        }
    }

    println!("Saving session...");
    match save_questions(&questions, file) {
        Ok(_) => println!("Session data saved!"),
        Err(_) => println!("Couldn't save session data"),
    }
}
