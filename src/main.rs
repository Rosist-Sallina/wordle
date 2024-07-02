use console;
use std::io::{self, Write};
use std::collections::HashMap;
use clap::{Arg , Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    if is_tty {
        let _ = tty();
    } else {
        let matches = Command::new("wordle")
            .version("0.1.0")
            .about("a simple wordle game")
            .arg(Arg::new("word")
                .short('w')
                .value_name("WORD")
                .help("Sets the word to guess")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("random")
                .short('r')
                .value_name("random")
                .help("random mode")
                .action(clap::ArgAction::SetFalse))
            .arg(Arg::new("difficult")
                .short('d')
                .value_name("difficult")
                .help("start difficult mode")
                .action(clap::ArgAction::SetFalse))
            .get_matches();
        
        let mut _flag = true;
        match matches.get_one::<String>("write") {                   //Judge the mode 1.write with word 2.write without word(input word in terminal) 3.random mode
            Some(write_value) => {
                if matches.get_flag("difficult") {
                    judge(&write_value);
                    _flag = false;
                    return Ok(());
                }
                else{
                    judge(&write_value);
                    _flag = false;
                }
            }
            None => {
                let mut line = String::new();
                io::stdin().read_line(&mut line)?;
                judge(&line);
                _flag = false;
            }
        }
        if matches.get_flag("random") {
            let line = get_useable_word();
            judge(&line);
            _flag = false;
        }

        if _flag
            {
                let mut line = String::new();
                io::stdin().read_line(&mut line)?;
                judge(&line);
            }           
    }
        Ok(())
}

mod builtin_words;
pub use builtin_words::select;     //Get built_in words

pub fn get_useable_word() -> String {
    select::get_useable_word()
}

pub fn get_available_word() -> String {
    select::get_available_word()
}

fn judge(str : &str){                                     //Judge fuction for get OJ output 
    let mut result = String::new();
    let mut map= HashMap::new();
    for c in str.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    } //Generate a map for word(to be guessed)'s color

    let mut _i = 0;
    while _i < 6{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.len() != str.len(){
            println!("INVALVD"); 
            continue;
        }

        let mut map_used = HashMap::new();  //how many (char,num) char we have used
        let mut i = 0;
        let mut char_color: HashMap<char ,char> = HashMap::new();  //the best result of the word

        for c in input.chars() {
            let count = map_used.entry(c).or_insert(0);
            *count += 1;

            if input.chars().nth(i) == str.chars().nth(i)  {
                result.push('G');
                char_color.insert(input.chars().nth(i).unwrap(), 'G');
                i += 1;
                continue;
            }
            if map.contains_key(&c) && map.get(&c) >= map_used.get(&c) {    //available char still in the word
                result.push('Y');
                if *char_color.get(&c).unwrap() == 'G' {
                    continue;
                }
                else{
                    char_color.insert(input.chars().nth(i).unwrap(), 'Y');
                }
                i += 1;
                continue;
            } 
            else {
                result.push('R');
                if *char_color.get(&c).unwrap() == 'G' || *char_color.get(&c).unwrap() == 'Y' {
                    continue;
                }
                else{
                    char_color.insert(input.chars().nth(i).unwrap(), 'R');
                }
                i += 1;
                continue;
            }        
        }

        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        for c in alphabet.chars() {
            if !char_color.contains_key(&c) {
                result.push('X');
            }
            else{
                result.push(*char_color.get(&c).unwrap());
            }
        }
        println!("{}", result);

        _i += 1;
    }
}

fn tty() -> Result<(), Box<dyn std::error::Error>>{
    println!(
        "I am in a tty. Please print {}!",
        console::style("colorful characters").bold().blink().blue()
    );
    print!("{}", console::style("Your name: ").bold().red());
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Welcome to wordle, {}!", line.trim());

    let mut line = String::new();
    print!("{}", console::style("Enter your guess: ").bold().red());
    io::stdin().read_line(&mut line)?;
    judge(&line); 


    // example: print arguments
    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!("");

    Ok(())
}