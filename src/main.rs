use console;
use core::panic;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use clap::{Arg, command, value_parser , ArgAction};
use std::collections::HashSet;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wordle", version = "0.1.0", about = "a simple wordle game")]
struct Args {
    /// Sets the word to guess
    #[arg(short, long, value_name = "WORD")]
    word: Option<String>,

    /// Random mode
    #[arg(short, long)]
    random: bool,

    /// Start difficult mode
    #[arg(short = 'D', long)]
    difficult: bool,

    /// Print the state of the game
    #[arg(short, long)]
    stats: bool,

    /// How many rounds you want to loop
    #[arg(short, long, value_name = "DAY")]
    day: Option<usize>,

    /// Seed for random
    #[arg(short, long, value_name = "SEED")]
    seed: Option<i32>,

    /// Final set of words
    #[arg(short = 'f', long, value_name = "FINAL_SET")]
    final_set: Option<String>,

    /// Acceptable set of words
    #[arg(short, long, value_name = "ACCEPTABLE_SET")]
    acceptable_set: Option<String>,

    /// Make the result into a json
    #[arg(short = 'S', long, value_name = "STATE")]
    state: Option<String>,

    /// Config file
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();

    // 处理解析后的参数
    if let Some(word) = args.word {
        println!("Word to guess: {}", word);
    }

    if args.random {
        println!("Random mode is on");
    }

    if args.difficult {
        println!("Difficult mode is on");
    }

    if args.stats {
        println!("Printing game stats");
    }

    if let Some(day) = args.day {
        println!("Number of rounds to loop: {}", day);
    }

    if let Some(seed) = args.seed {
        println!("Seed for random: {}", seed);
    }

    if let Some(final_set) = args.final_set {
        println!("Final set of words: {}", final_set);
    }

    if let Some(acceptable_set) = args.acceptable_set {
        println!("Acceptable set of words: {}", acceptable_set);
    }

    if let Some(state) = args.state {
        println!("State file: {}", state);
    }

    if let Some(config) = args.config {
        println!("Config file: {}", config);
    }

    // 其他逻辑处理...
}
