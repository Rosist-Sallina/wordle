use console;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use clap::{Arg , Command};
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref GLOBAL_HASHMAP: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    let mut count_success = 0;
    let mut count_played = 0;
    let mut answer_used = Vec::new();
    let mut count_success_loop = 0;
    let mut used_word_frequency = GLOBAL_HASHMAP.lock().unwrap();

    if is_tty {
        let _ = tty();
    } else {
        let mut success = 0;
        let mut w_mode = false;
        let mut answer = String::new();
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
            .arg(Arg::new("state")
                .short('s')
                .value_name("state")
                .help("print the state of the game")
                .action(clap::ArgAction::SetFalse))
            .arg(Arg::new("day")
                .short('d')
                .value_name("day")
                .help("how manys rounds you want to loop")
                .action(clap::ArgAction::SetTrue)
                .default_value("1"))
            .arg(Arg::new("seed")
                .short('s')
                .value_name("seed")
                .help("seed for random")
                .action(clap::ArgAction::SetTrue)
                .default_value("42"))
            .arg(Arg::new("final-set")
                .short('f')
                .value_name("final-set")
                .help("final set of words")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("acceptable-set")
                .short('a')
                .value_name("acceptable-set")
                .help("acceptable set of words")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("state")
                .short('s')
                .value_name("state")
                .help("make the result into a json")
                .default_value("state.json")
                .action(clap::ArgAction::SetTrue))
            .get_matches();
        
        let seed = matches.get_one::<u64>("seed").unwrap();
        let day = matches.get_one::<usize>("day").unwrap();
        let _final_set = matches.get_one::<String>("final-set").unwrap();
        let _acceptable_set = matches.get_one::<String>("acceptable-set").unwrap();

        let mut final_set = Vec::new();
        let mut acceptable_set = Vec::new();
        let mut temp1 = String::new();
        let mut temp2 = String::new();

        if matches.get_flag("final-set"){
            final_set = read_lines_from_file(_final_set, &mut temp1).unwrap();
        }
        else{
            final_set = select::FINAL.to_vec();
        }

        if matches.get_flag("acceptable-set"){
            acceptable_set = read_lines_from_file(_acceptable_set, &mut temp2).unwrap();
        }
        else{
            acceptable_set = select::ACCEPTABLE.to_vec();
        }

        if !is_subset(&final_set, &acceptable_set){
            println!("INVALVD");
            return Ok(());
        }

        let mut _flag = true;
        match matches.get_one::<String>("write") {                   //Judge the mode 1.write with word 2.write without word(input word in terminal) 3.random mode
            Some(write_value) => {
                answer = write_value.clone();
                if matches.get_flag("acceptable-set"){                                      //Enter word valid check
                    let mut temp = String::new();
                    let acceptable = read_lines_from_file(_acceptable_set, &mut temp).unwrap();
                    if !acceptable.contains(&write_value.as_str()){
                        println!("INVALVD");
                        return Ok(());
                    }
                }
                else{
                    let acceptable = select::ACCEPTABLE.to_vec();
                    if !acceptable.contains(&write_value.as_str()){
                        println!("INVALVD");
                        return Ok(());
                    }
                }
                success = judge(&write_value , matches.get_flag("difficult"));
                _flag = false;
                count_success_loop += success;
                count_played += 1;
                if success != 0{
                    count_success += 1;
                }
                success_judge(w_mode , success, answer);

                if matches.get_flag("state"){
                    print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                }
            }
            None => {
                loop{
                    if count_played != 0{                 //Check if player want another round
                        let mut _flag = true;
                        let mut line = String::new();
                        io::stdin().read_line(&mut line)?;
                        if line == "N"{
                            break;
                        }
                    }
                    let mut line = String::new();
                    io::stdin().read_line(&mut line)?;
                    if matches.get_flag("acceptable-set"){                                      //Enter word valid check
                        let mut temp = String::new();
                        let acceptable = read_lines_from_file(_acceptable_set, &mut temp).unwrap();
                        if !acceptable.contains(&line.as_str()){
                            println!("INVALVD");
                            return Ok(());
                        }
                    }
                    else{
                        let acceptable = select::ACCEPTABLE.to_vec();
                        if !acceptable.contains(&line.as_str()){
                            println!("INVALVD");
                            return Ok(());
                        }
                    }
                    success = judge(&line , matches.get_flag("difficult"));
                    _flag = false;
                    w_mode = true;
                    answer = line;

                    success_judge(w_mode , success, answer.clone());
                    count_played += 1;
                    if success != 0{
                        count_success += 1;
                        count_success_loop += success;
                    }

                    if matches.get_flag("state"){
                        print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                    }
                }
            }
        }
        if matches.get_flag("random") {
            loop{
                let mut line = String::new();
                if matches.get_flag("final-set"){
                    line = get_useable_word_file(*day , *seed , _final_set);
                }
                else{
                    line = get_useable_word_default(*day , *seed);
                }
                if answer_used.contains(&line){                     //Check if the word has been used
                    continue;
                }
                if count_played != 0{                 //Check if player want another round
                    let mut _flag = true;
                    let mut line = String::new();
                    io::stdin().read_line(&mut line)?;
                    if line == "N"{
                        break;
                    }
                }
                success = judge(&line , matches.get_flag("difficult"));
                _flag = false;
                answer = line;
                success_judge(w_mode , success, answer.clone());
                answer_used.push(answer.clone());
                count_played += 1;
                if success != 0{
                    count_success += 1;
                    count_success_loop += success;            
                    }
                }

                if matches.get_flag("state"){
                    print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                }
        }
        if _flag{                                    //default mode
            loop{
                    if count_played != 0{                 //Check if player want another round
                        let mut _flag = true;
                        let mut line = String::new();
                        io::stdin().read_line(&mut line)?;
                        if line == "N"{
                            break;
                        }
                    }
                    let mut line = String::new();
                    io::stdin().read_line(&mut line)?;
                    if matches.get_flag("acceptable-set"){                                      //Enter word valid check
                        let mut temp = String::new();
                        let acceptable = read_lines_from_file(_acceptable_set, &mut temp).unwrap();
                        if !acceptable.contains(&line.as_str()){
                            println!("INVALVD");
                            return Ok(());
                        }
                    }
                    else{
                        let acceptable = select::ACCEPTABLE.to_vec();
                        if !acceptable.contains(&line.as_str()){
                            println!("INVALVD");
                            return Ok(());
                        }
                    }
                    success = judge(&line , matches.get_flag("difficult"));
                    _flag = false;
                    answer = line;
                    success_judge(w_mode , success, answer);
                    count_played += 1;
                    if success != 0{
                        count_success += 1;
                        count_success_loop += success;
                    }

                    if matches.get_flag("state"){
                        print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                    }
            }
            }

       
    }
        Ok(())
}

mod builtin_words;
pub use builtin_words::select;     //Get built_in words

fn judge(str : &str , flag: bool) -> i32{                         //All judge function
    let mut result = String::new();
    let mut default_map = HashMap::new();

    for c in str.chars() {
        let count = default_map.entry(c).or_insert(0);
        *count += 1;
    } //Generate a map for word(to be guessed)'s color  # default
    
    let mut _i = 0;
    while _i < 6{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.len() != str.len(){
            println!("INVALVD"); 
            continue;
        }
        else if flag && _dmode_vavid_check(str, &input, &result){
            println!("INVALVD");
            continue;
        }
        
        let mut used_word_frequency = GLOBAL_HASHMAP.lock().unwrap();
        used_word_frequency.entry(input.clone()).or_insert(0);

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
            if default_map.contains_key(&c) && default_map.get(&c) >= map_used.get(&c) {    //available char still in the word
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

        let mut flag = false;
        for c in result.chars() {
            if c != 'G' {
                flag = true;
                break;
            }
        }
        _i += 1;
        if flag{
            return _i;
        }
    }
    return 0;
}

fn _dmode_vavid_check(str : &str , input : &String , result : &String) -> bool {
    let str = &str[0..5];
    let mut yellow = Vec::new();

    for ((c_default , c_input ), c_result) in str.chars().zip(input.chars()).zip(result.chars()) {
        if c_result == 'G' && c_default != c_input {
            return false;
        }
        if c_result == 'Y'{
            yellow.push(c_default);
        }
    }

    for c in yellow {
        if !input.contains(c) {
            return false;
        }
    }

    true
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
    judge(&line , false); 


    // example: print arguments
    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!("");

    Ok(())
}

fn success_judge(flag:bool , success : i32 , answer : String){
    if success != 0{
        println!("CORRECT {:?}" , success);
    }
    else {
        println!("FAIL {:?}" , answer)
    }

    if flag{
        println!("Another round? (y/n)");
    }
}

fn print_state(count_success : i32 , count_played : i32 , count_success_loop : i32 , used_word_frequency : HashMap<String , i32>){
    println!("{} {} {}" , count_success , count_played-count_success , count_success_loop / count_success);

    let vec = hash_map_sort(used_word_frequency);
    let vec = vec.iter().take(5);
    for (key , value) in vec{
        println!("{} {}" , key , value);
    }
}

fn hash_map_sort(used_word_frequency : HashMap<String , i32>) -> Vec<(String , i32)>{
    let mut vec = Vec::new();
    for (key , value) in used_word_frequency.iter(){
        vec.push((key.clone() , value.clone()));
    }
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec
}

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::path::Path;

fn get_useable_word_default(day : usize, seed : u64) -> String{                    //Get the word from the default set
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vec = select::FINAL.to_vec();
    vec.shuffle(&mut rng);
    vec[day-1].to_string()
}

fn get_useable_word_file(day:usize, seed:u64,final_set:&str) -> String{  //Get the word from the file
    let mut buffer = String::new();
    let mut vec = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);
    vec = read_lines_from_file(final_set, &mut buffer).unwrap();
    vec.shuffle(&mut rng);
    vec[day-1].to_string()
}

fn read_lines_from_file<'a, P>(filename: P, buffer: &'a mut String) -> io::Result<Vec<&'a str>>     //read the file from a file and change all the word to the upper state
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    buffer.clear();
    for line in reader.lines() {
        let line = line?.to_uppercase();
        buffer.push_str(&line);
        buffer.push('\n');
    }

    let lines: Vec<&str> = buffer.lines().collect();
    if !wordbox_valid_check(lines.to_vec()){
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"));
    }
    Ok(lines)
}

fn wordbox_valid_check(input : Vec<&str>) -> bool{   //check if input is valid
    let mut vec = Vec::new();
    for i in 0..input.len(){
        if vec.contains(&input[i]){
            return false;
        }
        if input[i].len() != 5{
            return false;
        }
        for j in 0..input[i].len(){
            if !input[i].chars().nth(j).unwrap().is_alphabetic(){
                return false;
            }
        }
        vec.push(input[i]);
    }
    true
}

fn is_subset<T: Eq + std::hash::Hash>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {         //子集判断函数 ， 资料来源于blog
    let set1: HashSet<_> = vec1.iter().collect();
    let set2: HashSet<_> = vec2.iter().collect();
    set1.is_subset(&set2)
}
