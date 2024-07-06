use console;
use core::panic;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use clap::{arg, command, value_parser, ArgAction};
use std::collections::HashSet;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    let mut count_success = 0;
    let mut count_played = 0;
    let mut answer_used = Vec::new();
    let mut count_success_loop = 0;
    let mut used_word_frequency = HashMap::new();

    if is_tty {
        let _ = tty();
    } else {
        // let mut success = 0;
        let mut w_mode = false;
        let mut answer = String::new();
        let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -w --word <WORD> "Sets the word to guess"
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -r --random "Random mode"
            )
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -D --difficult "Start difficult mode"
            )
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -t --stats "Print the state of the game"
            )
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -d --day <DAY> "How many rounds you want to loop"
            )
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -s --seed <SEED> "Seed for random"
            )
            .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(
                -f --final_set <FINAL_SET> "Final set of words"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -a --acceptable_set <ACCEPTABLE_SET> "Acceptable set of words"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -S --state <STATE> "Make the result into a JSON"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -c --config <CONFIG> "Config file"
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .get_matches();
        
        let mut default_config = Config{
            random : Some(false),
            difficult : Some(false),
            stats : Some(false),
            day : Some(1),
            seed : Some(42),
            final_set : Some("".to_string()),
            acceptable_set : Some("".to_string()),
            state : Some("".to_string()),
            word : Some("".to_string()),
        };
        if let Some(config) = matches.get_one::<String>("config"){
            default_config = json_to_config(config.to_string()).unwrap();
        }
        if let Some(seed) = matches.get_one::<i32>("seed"){
            default_config.seed = Some(*seed);
        }
        if let Some(day) = matches.get_one::<usize>("day"){
            default_config.day = Some(*day);
        }
        if matches.get_flag("difficult"){
            default_config.difficult = Some(true);
        }
        if matches.get_flag("random"){
            default_config.random = Some(true);
        }
        if matches.get_flag("stats"){
            default_config.stats = Some(true);
        }
        if let Some(final_set)= matches.get_one::<String>("final_set"){
            default_config.final_set = Some(final_set.clone());
        }
        if let Some(acceptable_set) = matches.get_one::<String>("acceptable_set"){
            default_config.acceptable_set = Some(acceptable_set.clone());
        }
        if let Some(state) = matches.get_one::<String>("state"){
            default_config.state = Some(state.clone());
        }
        if let Some(word) = matches.get_one::<String>("word"){
            default_config.word = Some(word.clone());
        }
        
        let mut final_set = Vec::new();
        let mut acceptable_set = Vec::new();
        let mut temp1 = String::new();
        let mut temp2 = String::new();

        if matches.contains_id("final_set"){
            final_set = read_lines_from_file(default_config.final_set.clone().unwrap(), &mut temp1).unwrap();
        }
        else{
            final_set = select::FINAL.to_vec();
        }

        if matches.contains_id("acceptable_set"){
            acceptable_set = read_lines_from_file(&default_config.acceptable_set.unwrap(), &mut temp2).unwrap();
        }
        else{
            acceptable_set = select::ACCEPTABLE.to_vec();
        }

        if matches.contains_id("state"){
            let data = fs::read_to_string(&default_config.state.unwrap()).unwrap();
            let _json : State = serde_json::from_str(&data).unwrap();
        }

        if !is_subset(&final_set, &acceptable_set){
            panic!("INVALID");
        }

        let mut _flag = true;

        if matches.get_flag("random") && matches.contains_id("word"){
            panic!("INVALID")
        }
        if matches.contains_id("word") && (matches.contains_id("seed") || matches.contains_id("day")){
            panic!("INVALID")
        }

        if matches.contains_id("word"){
            match matches.get_one::<String>("word") {                   //Judge the mode 1.write with word 2.write without word(input word in terminal) 3.random mode
                Some(write_value) => {
                    answer = write_value.clone();
                    if !acceptable_set.contains(&answer.as_str()){
                        println!("INVALID");
                        return Ok(());
                    }
                    let  (success, _gusses ,frequency) = judge(&answer , default_config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
                    used_word_frequency = frequency;
                    _flag = false;
                    count_success_loop += success;
                    count_played += 1;
                    if success != 0{
                        count_success += 1;
                    }
                    success_judge(w_mode , success, answer);

                    if default_config.stats.unwrap(){
                        print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                    }
                }
                None => {
                    loop{
                        if count_played != 0{                 //Check if player want another round
                            let mut _flag = true;
                            let mut line = String::new();
                            io::stdin().read_line(&mut line)?;
                            if line == "N\n" || line == "n\n"{
                                break;
                            }
                        }
                        let mut line = String::new();
                        io::stdin().read_line(&mut line)?;
                        line.pop();
                        if !acceptable_set.contains(&line.as_str()){
                            println!("INVALID");
                            return Ok(());
                        }
                        let (success , _guess ,frequency) = judge(&line , default_config.difficult.unwrap(), used_word_frequency.clone() , &acceptable_set);
                        used_word_frequency = frequency;
                        _flag = false;
                        w_mode = true;
                        answer = line;

                        success_judge(w_mode , success, answer.clone());
                        count_played += 1;
                        if success != 0{
                            count_success += 1;
                            count_success_loop += success;
                        }

                        if default_config.stats.unwrap(){
                            print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                        }
                    }
                }
            }
        }
        if default_config.random.unwrap() && _flag{
            loop{
                let mut line = String::new();
                if matches.contains_id("final_set"){
                    line = get_useable_word_file(default_config.day.unwrap(), default_config.seed.unwrap().try_into().unwrap(), default_config.final_set.clone().unwrap().as_str());
                }
                else{
                    line = get_useable_word_default(default_config.day.unwrap(), default_config.seed.unwrap().try_into().unwrap());
                }
                if answer_used.contains(&line){                     //Check if the word has been used
                    continue;
                }
            
                let (success , gusses , frequency) = judge(&line , default_config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
                used_word_frequency = frequency;
                _flag = false;
                answer = line;
                success_judge(w_mode , success, answer.clone());
                answer_used.push(answer.clone());
                count_played += 1;
                if success != 0{
                    count_success += 1;
                    count_success_loop += success;            
                    }
                if default_config.stats.unwrap(){
                    print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                }
                if matches.contains_id("state"){
                    let _ = state_to_json(matches.get_one::<String>("state").unwrap().clone() , answer.clone() , gusses.clone());
                }
                    
                if count_played != 0{                 //Check if player want another round
                    let mut _flag = true;
                    let mut line = String::new();
                    io::stdin().read_line(&mut line)?;
                    if line == "N\n" || line == "n\n"{
                        break;
                    }
                }

                default_config.day = Some(default_config.day.unwrap() + 1);
            }
        }
        if _flag{                                    //default mode
            loop{
                    if count_played != 0{                 //Check if player want another round
                        let mut _flag = true;
                        let mut line = String::new();
                        io::stdin().read_line(&mut line)?;
                        if line == "N\n" || line == "n\n"{
                            break;
                        }
                    }
                    let mut line = String::new();
                    io::stdin().read_line(&mut line)?;
                    line.pop();
                    if !acceptable_set.contains(&line.as_str()){
                        println!("INVALID");
                        return Ok(());
                    }
                    let (success , _gusses ,frequency) = judge(&line , default_config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
                    used_word_frequency = frequency;
                    _flag = false;
                    answer = line;
                    success_judge(w_mode , success, answer);
                    count_played += 1;
                    if success != 0{
                        count_success += 1;
                        count_success_loop += success;
                    }

                    if default_config.stats.unwrap(){
                        print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                    }
            }
            }

       
    }
        Ok(())
}

mod builtin_words;
pub use builtin_words::select;     //Get built_in words

fn judge(str : &str , flag: bool , mut used_word_frequency : HashMap<String , i32> , acceptable_set : &Vec<&str>) -> (i32 , Vec<String> ,HashMap<String , i32>){                         //All judge function
    let mut default_map = HashMap::new();
    let mut gusses = Vec::new();
    let mut _result = String::new();
    let mut last = String::from("");

    for c in str.chars() {
        let count = default_map.entry(c).or_insert(0);
        *count += 1;
    } //Generate a map for word(to be guessed)'s color  # default
    
    let mut _i = 0;
    let mut char_color: HashMap<char ,char> = HashMap::new();  //the best result of the word
    while _i < 6{
        let mut result = String::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        
        if input.len() != str.len(){
            println!("INVALID"); 
            continue;
        }
        else if flag && !_dmode_vavid_check(&last, &input, &_result){
            println!("INVALID");
            continue;
        }
        else if !acceptable_set.contains(&input.as_str()){
            println!("INVALID");
            continue;
        }

        last = input.clone();       
        used_word_frequency.entry(input.clone()).or_insert(0);
        if used_word_frequency.contains_key(&input){
            let count = used_word_frequency.entry(input.clone()).or_insert(0);
            *count += 1;
        }
        gusses.push(input.clone().to_uppercase());

        let mut map_used = HashMap::new();  //how many (char,num) char we have used
        let mut i = 0;

        for c in input.chars(){
            if input.chars().nth(i) == str.chars().nth(i){
                let count = map_used.entry(c).or_insert(0);
                *count += 1;
                result.push('G');
                char_color.insert(input.chars().nth(i).unwrap(), 'G');
                i += 1;
                continue;
            }
            else {
                result.push(' ');
                i += 1;
            }
        }
        i = 0;
        for c in input.chars() {
            if result.chars().nth(i).unwrap() == 'G' {
                i += 1;
                continue;
            }
            if default_map.contains_key(&c) && default_map.get(&c) > map_used.get(&c) {    //available char still in the word
                let count = map_used.entry(c).or_insert(0);
                *count += 1;
                result = fix_string_by_index(&result , i , 'Y');
                if char_color.contains_key(&c) && *char_color.get(&c).unwrap() == 'G' {
                    i += 1;
                    continue;
                }
                else{
                    char_color.insert(input.chars().nth(i).unwrap(), 'Y');
                }
                i += 1;
                continue;
            } 
            else {
                let count = map_used.entry(c).or_insert(0);
                *count += 1;
                result = fix_string_by_index(&result , i , 'R');
                if char_color.contains_key(&c) && (*char_color.get(&c).unwrap() == 'G' || *char_color.get(&c).unwrap() == 'Y') {
                    i += 1;
                    continue;
                }
                else{
                    char_color.insert(input.chars().nth(i).unwrap(), 'R');
                }
                i += 1;
                continue;
            }        
        }

        result.push(' ');
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        for c in alphabet.chars() {
            if !char_color.contains_key(&c) {
                result.push('X');
            }
            else{
                result.push(*char_color.get(&c).unwrap());
            }
        }
        println!("{}", result);
        _result = result.clone().chars().take(5).collect();
        
        let first_five = &result[0..5];
        let mut flag = false;
        for c in first_five.chars() {
            if c != 'G' {
                flag = true;
                break;
            }
        }
        _i += 1;
        if !flag{
            return (_i , gusses , used_word_frequency);
        }
    }
    return (0 , gusses , used_word_frequency);
}

fn _dmode_vavid_check(str : &str , input : &String , result : &String) -> bool {
    let mut yellow = Vec::new();

    if str == ""{
        return true;
    }
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
    let used_word_frequency = HashMap::new();
    judge(&line , false , used_word_frequency.clone() , &select::ACCEPTABLE.to_vec()); 

    // example: print arguments
    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!("");

    Ok(())
}

fn success_judge(_flag:bool , success : i32 , answer : String){
    if success != 0{
        println!("CORRECT {}" , success);
    }
    else {
        println!("FAILED {}" , answer.to_uppercase())
    }
}

fn print_state(count_success: i32, count_played: i32, count_success_loop: i32, used_word_frequency: HashMap<String, i32>) {
    if count_success != 0 {
        let success_rate = count_success_loop as f64 / count_success as f64;
        println!("{} {} {:.2}", count_success, count_played - count_success, success_rate);
    } else {
        println!("{} {} 0.00", count_success, count_played - count_success);
    }

    let mut vec = hash_map_sort(used_word_frequency);
    vec = vec.iter().take(5).cloned().collect();
    for (key, value) in vec {
        print!("{} {} ", key.to_uppercase(), value);
    }
    print!("\n");
    io::stdout().flush().unwrap();
}

fn hash_map_sort(map: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut vec: Vec<(String, i32)> = map.into_iter().collect();
    vec.sort_by(|a, b| {
        // 先按值降序排序
        let value_cmp = b.1.cmp(&a.1);
        if value_cmp == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            value_cmp
        }
    });
    vec
}

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::fs::{self, File};
use std::path::Path;

fn get_useable_word_default(day : usize, seed : u64) -> String{                    //Get the word from the default set
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vec = select::FINAL.to_vec();
    vec.shuffle(&mut rng);
    vec[day-1].to_string()
}

fn get_useable_word_file(day:usize, seed:u64,final_set:&str) -> String{  //Get the word from the file
    let mut buffer = String::new();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vec = read_lines_from_file(final_set, &mut buffer).unwrap();
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
        let line = line?.to_lowercase();
        buffer.push_str(&line);
        buffer.push('\n');
    }

    let mut lines: Vec<&str> = buffer.lines().collect();
    if !wordbox_valid_check(lines.to_vec()){
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"));
    }
    lines.sort();
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

#[derive(Serialize, Deserialize)]
struct Game{
    answer : Option<String>,
    guesses : Option<Vec<String>>,
}
#[derive(Serialize, Deserialize)]
struct State{
    total_rounds:Option<i32> , 
    games : Option<Vec<Game>>,
}

use serde::{Serialize, Deserialize};
fn state_to_json(path:String , answer:String , guesses:Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let data = fs::read_to_string(&path).unwrap();
    let mut json = State{
        total_rounds : Some(0),
        games : Some(Vec::new()),
    };
    if data != "" {
        json = serde_json::from_str(&data).unwrap();
    }
    let temp_game = Game{
        answer : Some(answer.to_uppercase()),
        guesses : Some(guesses),
    };
    json.games.as_mut().unwrap().push(temp_game);
    json.total_rounds = Some(json.total_rounds.unwrap() + 1);
    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create(&path)?;
    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

fn json_to_config(path:String) -> Result<Config, Box<dyn std::error::Error>>{
    let data = fs::read_to_string(&path).unwrap();
    let json : Config = serde_json::from_str(&data).unwrap();
    Ok(json)
}

#[derive(Serialize, Deserialize)]
struct Config{
    random : Option<bool> ,
    difficult : Option<bool> ,
    stats : Option<bool> , 
    day : Option<usize>,
    seed : Option<i32>,
    final_set : Option<String> ,
    acceptable_set : Option<String> ,
    state : Option<String> ,
    word : Option<String>,
}

fn fix_string_by_index(input : &str , index : usize , c : char) -> String{
    let mut result = String::new();
    for i in 0..input.len(){
        if i == index{
            result.push(c);
        }
        else{
            result.push(input.chars().nth(i).unwrap());
        }
    }
    result
}
