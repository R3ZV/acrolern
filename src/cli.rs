use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Displays a short description about the acronym after each answer
    #[arg(short, long)]
    pub desc: Option<bool>,

    /// Uses only the questions that have the score at most "upto"
    #[arg(short, long)]
    pub upto: Option<usize>,

    /// Uses only the questions that have matching tags
    #[arg(short, long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Starts the quiz using the options given and starting from the lowest scores
    Play,

    /// Displays all the availabe tags that the user can filter by
    Tags,
}
