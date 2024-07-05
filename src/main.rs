use console;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use clap::{Arg, Command};
use std::collections::HashSet;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    let mut count_success = 0;
    let mut count_played = 0;
    let mut answer_used = Vec::new();
    let mut count_success_loop = 0;
    let mut used_word_frequency = HashMap::new();

    if !is_tty {
        let _ = tty();
    } else {
        // let mut success = 0;
        let mut w_mode = false;
        let mut answer = String::new();
        let matches = Command::new("wordle")
        .version("0.1.0")
        .about("a simple wordle game")
        .arg(Arg::new("word")
            .short('w')
            .value_name("WORD")
            .help("Sets the word to guess")
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("random")
            .short('r')
            .help("random mode"))
        .arg(Arg::new("difficult")
            .short('D')
            .help("start difficult mode"))
        .arg(Arg::new("stats")
            .short('t')
            .help("print the state of the game"))
        .arg(Arg::new("day")
            .short('d')
            .value_name("DAY")
            .help("how many rounds you want to loop")
            .value_parser(clap::value_parser!(usize)))
        .arg(Arg::new("seed")
            .short('s')
            .value_name("SEED")
            .help("seed for random")
            .value_parser(clap::value_parser!(i32)))
        .arg(Arg::new("final-set")
            .short('f')
            .long("final-set")
            .value_name("FINAL-SET")
            .help("final set of words")
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("acceptable-set")
            .short('a')
            .long("acceptable-set")
            .value_name("ACCEPTABLE-SET")
            .help("acceptable set of words")
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("state")
            .short('S')
            .value_name("STATE")
            .help("make the result into a json")
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("config")
            .short('c')
            .value_name("CONFIG")
            .help("config file")
            .value_parser(clap::value_parser!(String)))
        .get_matches();
        
        let mut default_config = Config{
            random : true,
            difficult : false,
            stats : false,
            day : 1,
            seed : 42,
            final_set : "".to_string(),
            acceptable_set : "".to_string(),
            state : "".to_string(),
            word : "".to_string(),
        };
        if let Some(config) = matches.get_one::<String>("config"){
            default_config = json_to_config(config.to_string()).unwrap();
        }
        if let Some(seed) = matches.get_one::<i32>("seed"){
            default_config.seed = *seed;
        }
        if let Some(day) = matches.get_one::<usize>("day"){
            default_config.day = *day;
        }
        if matches.contains_id("difficult"){
            default_config.difficult = true;
        }
        if matches.contains_id("random"){
            default_config.random = true;
        }
        if matches.contains_id("stats"){
            default_config.stats = true;
        }
        if let Some(final_set)= matches.get_one::<String>("final-set"){
            default_config.final_set = final_set.clone();
        }
        if let Some(acceptable_set) = matches.get_one::<String>("acceptable-set"){
            default_config.acceptable_set = acceptable_set.clone();
        }
        if let Some(state) = matches.get_one::<String>("state"){
            default_config.state = state.clone();
        }
        if let Some(word) = matches.get_one::<String>("word"){
            default_config.word = word.clone();
        }
        
        let mut final_set = Vec::new();
        let mut acceptable_set = Vec::new();
        let mut temp1 = String::new();
        let mut temp2 = String::new();

        if matches.contains_id("final-set"){
            final_set = read_lines_from_file(&default_config.final_set, &mut temp1).unwrap();
        }
        else{
            final_set = select::FINAL.to_vec();
        }

        if matches.contains_id("acceptable-set"){
            acceptable_set = read_lines_from_file(&default_config.acceptable_set, &mut temp2).unwrap();
        }
        else{
            acceptable_set = select::ACCEPTABLE.to_vec();
        }

        if !is_subset(&final_set, &acceptable_set){
            println!("INVALVD");
            return Ok(());
        }

        let mut _flag = true;
        if matches.contains_id("word"){
            match matches.get_one::<String>("word") {                   //Judge the mode 1.write with word 2.write without word(input word in terminal) 3.random mode
                Some(write_value) => {
                    answer = write_value.clone();
                    answer.pop();
                    if acceptable_set.contains(&answer.as_str()){
                        println!("INVALVD");
                        return Ok(());
                    }
                    let  (success, _gusses ,frequency) = judge(&write_value , default_config.difficult , used_word_frequency.clone());
                    used_word_frequency = frequency;
                    _flag = false;
                    count_success_loop += success;
                    count_played += 1;
                    if success != 0{
                        count_success += 1;
                    }
                    success_judge(w_mode , success, answer);

                    if default_config.stats{
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
                        if acceptable_set.contains(&line.as_str()){
                            println!("INVALVD");
                            return Ok(());
                        }
                        let (success , _guess ,frequency) = judge(&line , default_config.difficult , used_word_frequency.clone());
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

                        if default_config.stats{
                            print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                        }
                    }
                }
            }
        }
        if default_config.random {
            loop{
                let mut line = String::new();
                if matches.contains_id("final-set"){
                    line = get_useable_word_file(default_config.day , default_config.seed.try_into().unwrap(), &default_config.final_set.as_str());
                }
                else{
                    line = get_useable_word_default(default_config.day , default_config.seed.try_into().unwrap());
                }
                if answer_used.contains(&line){                     //Check if the word has been used
                    continue;
                }
            
                let (success , gusses , frequency) = judge(&line , default_config.difficult , used_word_frequency.clone());
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
                if default_config.stats{
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

                default_config.day += 1;
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
                        println!("INVALVD");
                        return Ok(());
                    }
                    let (success , _gusses ,frequency) = judge(&line , default_config.difficult , used_word_frequency.clone());
                    used_word_frequency = frequency;
                    _flag = false;
                    answer = line;
                    success_judge(w_mode , success, answer);
                    count_played += 1;
                    if success != 0{
                        count_success += 1;
                        count_success_loop += success;
                    }

                    if default_config.stats{
                        print_state(count_success , count_played , count_success_loop , used_word_frequency.clone());
                    }
            }
            }

       
    }
        Ok(())
}

mod builtin_words;
pub use builtin_words::select;     //Get built_in words

fn judge(str : &str , flag: bool , mut used_word_frequency : HashMap<String , i32>) -> (i32 , Vec<String> ,HashMap<String , i32>){                         //All judge function
    let mut default_map = HashMap::new();
    let mut gusses = Vec::new();

    for c in str.chars() {
        let count = default_map.entry(c).or_insert(0);
        *count += 1;
    } //Generate a map for word(to be guessed)'s color  # default
    
    let mut _i = 0;
    while _i < 6{
        let mut result = String::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();

        if input.len() != str.len(){
            println!("INVALVD"); 
            continue;
        }
        else if flag && _dmode_vavid_check(str, &input, &result){
            println!("INVALVD");
            continue;
        }
        
        used_word_frequency.entry(input.clone()).or_insert(0);
        if used_word_frequency.contains_key(&input){
            let count = used_word_frequency.entry(input.clone()).or_insert(0);
            *count += 1;
        }
        gusses.push(input.clone());

        let mut map_used = HashMap::new();  //how many (char,num) char we have used
        let mut i = 0;
        let mut char_color: HashMap<char ,char> = HashMap::new();  //the best result of the word

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
    let used_word_frequency = HashMap::new();
    judge(&line , false , used_word_frequency.clone()); 

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
        println!("CORRECT {}" , success);
    }
    else {
        println!("FAIL {}" , answer)
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

#[derive(Serialize, Deserialize)]
struct Game{
    name : String,
    gusses : Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct State{
    total_rounds:i32 , 
    games : Vec<Game>,
}

use serde::{Serialize, Deserialize};
fn state_to_json(path:String , answer:String , gusses:Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let data = fs::read_to_string(&path)?;
    let mut json : State = serde_json::from_str(&data)?;
    let temp_game = Game{
        name : answer,
        gusses : gusses,
    };
    json.games.push(temp_game);
    json.total_rounds += 1;
    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create(&path)?;
    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

fn json_to_config(path:String) -> Result<Config, Box<dyn std::error::Error>>{
    let data = fs::read_to_string(&path)?;
    let json : Config = serde_json::from_str(&data)?;
    Ok(json)
}

#[derive(Serialize, Deserialize)]
struct Config{
    random : bool ,
    difficult : bool ,
    stats : bool , 
    day : usize,
    seed : i32,
    final_set : String ,
    acceptable_set : String ,
    state : String ,
    word : String,
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
