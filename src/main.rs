use console;
use std::io::{self, Write};
use std::collections::HashMap;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
    } else {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let result = judge(&line);
        println!("{}", result);
    }

    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Welcome to wordle, {}!", line.trim());

    let mut line = String::new();
    print!("{}", console::style("Enter your guess: ").bold().red());
    io::stdin().read_line(&mut line)?;
    let result = judge(&line);

    println!("{}", result);


    // example: print arguments
    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!("");

    Ok(())
}

mod builtin_words;

pub use builtin_words::select;

pub fn get_useable_word() -> String {
    select::get_useable_word()
}

pub fn get_available_word() -> String {
    select::get_available_word()
}

fn judge(str : &str) -> String {
    let useable_word = get_useable_word();
    let _available_word = get_available_word();
    let mut result = String::new();

    let mut map= HashMap::new();

    for c in useable_word.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut map_used = HashMap::new();
    let mut i = 0;

    let mut char_color: HashMap<char ,char> = HashMap::new();

    for c in str.chars() {
        let count = map_used.entry(c).or_insert(0);
        *count += 1;

        if str.chars().nth(i) == useable_word.chars().nth(i)  {
            result.push('G');
            char_color.insert(str.chars().nth(i).unwrap(), 'G');
            i += 1;
            continue;
        }
        if map.contains_key(&c) && map.get(&c) >= map_used.get(&c) {
            result.push('Y');
            if *char_color.get(&c).unwrap() == 'G' {
                continue;
            }
            else{
                char_color.insert(str.chars().nth(i).unwrap(), 'Y');
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
                char_color.insert(str.chars().nth(i).unwrap(), 'R');
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

    result
}
