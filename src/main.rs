use console;
use core::panic;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use clap::{Arg, command, value_parser , ArgAction};
use std::collections::HashSet;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    let mut count_success = 0;
    let mut count_played = 0;
    let mut count_success_loop = 0;

    if is_tty {
        Ok(())
    } else {
        // let mut success = 0;
        let mut w_mode = false;
        let mut answer = String::new();
        let matches = command!() // requires `cargo` feature
        .version("0.1.0")
        .about("a simple wordle game")
        .arg(Arg::new("word")
            .short('w')
            .long("word")
            .value_name("WORD")
            .help("Sets the word to guess")
            .required(false)
            .num_args(0..=1)
            .value_parser(value_parser!(String)))
        .arg(Arg::new("random")
            .short('r')
            .long("random")
            .help("random mode")
            .required(false)
            .action(ArgAction::SetTrue))
        .arg(Arg::new("difficult")
            .short('D')
            .long("difficult")
            .help("start difficult mode")
            .required(false)
            .action(ArgAction::SetTrue))
        .arg(Arg::new("stats")
            .short('t')
            .long("stats")
            .help("print the state of the game")
            .action(ArgAction::SetTrue)
            .required(false))
        .arg(Arg::new("day")
            .short('d')
            .long("day")
            .value_name("DAY")
            .help("how many rounds you want to loop")
            .required(false)
            .value_parser(value_parser!(usize)))
        .arg(Arg::new("seed")
            .short('s')
            .long("seed")
            .value_name("SEED")
            .help("seed for random")
            .required(false)
            .value_parser(value_parser!(i32)))
        .arg(Arg::new("final-set")
            .short('f')
            .long("final-set")
            .value_name("FINAL_SET")
            .help("final set of words")
            .required(false)
            .value_parser(value_parser!(String)))
        .arg(Arg::new("acceptable-set")
            .short('a')
            .long("acceptable-set")
            .value_name("ACCEPTABLE_SET")
            .help("acceptable set of words")
            .required(false)
            .value_parser(value_parser!(String)))
        .arg(Arg::new("state")
            .short('S')
            .long("state")
            .value_name("STATE")
            .help("make the result into a json")
            .required(false)
            .value_parser(value_parser!(String)))
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("CONFIG")
            .help("config file")
            .required(false)
            .value_parser(value_parser!(String)))
        .get_matches();        
        Ok(())
    }
}

