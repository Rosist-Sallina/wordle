pub mod crate_judge{
    use std::{collections::HashMap, io::Write};
    use std::io;
    use ansi_term::Colour::{Green,Red,Yellow};

    use crate::fix_string_by_index;
    pub fn judge(str : &str , flag: bool , mut used_word_frequency : HashMap<String , i32> , acceptable_set : &Vec<&str>) -> (i32 , Vec<String> ,HashMap<String , i32>){                         //All judge function
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
            else if flag && !crate::_dmode_vavid_check(&last, &input, &_result){
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
                    result = crate::fix_string_by_index(&result , i , 'Y');
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
                    result = crate::fix_string_by_index(&result , i , 'R');
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

    pub fn judge_tty(str : &str , flag: bool , mut used_word_frequency : HashMap<String , i32> , acceptable_set : &Vec<&str>) -> (i32 , Vec<String> ,HashMap<String , i32>){                         //All judge function
        let mut default_map = HashMap::new();
        let mut gusses = Vec::new();
        let mut _result = String::new();
        let mut last = String::from("");
    
        for c in str.chars() {
            let count = default_map.entry(c).or_insert(0);
            *count += 1;
        } //Generate a map for word(to be guessed)'s color  # default
        println!("Please enter your guess word:");
        let mut _i = 0;
        let mut char_color: HashMap<char ,char> = HashMap::new();  //the best result of the word
        while _i < 6{
            let mut result = String::new();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.pop();
            
            if input.len() != str.len(){
                println!("{}", Red.paint("Error : INVALID INPUT")); 
                continue;
            }
            else if flag && !crate::_dmode_vavid_check(&last, &input, &_result){
                println!("{}", Red.paint("Error : INVALID INPUT")); 
                continue;
            }
            else if !acceptable_set.contains(&input.as_str()){
                println!("{}", Red.paint("Error : INVALID INPUT")); 
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
                    result = crate::fix_string_by_index(&result , i , 'R');
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
            _result = result.clone().chars().take(5).collect();
            for (char , color) in input.chars().zip(_result.chars()){
                if color == 'G'{
                    print!("{}", Green.paint(char.to_string()));
                }
                else if color == 'Y'{
                    print!("{}", Yellow.paint(char.to_string()));
                }
                else if color == 'R'{
                    print!("{}", Red.paint(char.to_string()));
                }
            }
            print!("\n");
            print_alphabet(char_color.clone());  
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

    fn print_alphabet(char_color : HashMap<char , char>){
        let first_line = "qwertyuiop";
        let second_line = "asdfghjkl";
        let third_line = "zxcvbnm";

        for c in first_line.chars(){
            if char_color.contains_key(&c){
                if *char_color.get(&c).unwrap() == 'G'{
                    print!("{}", Green.paint(c.to_string()));
                }
                else if *char_color.get(&c).unwrap() == 'Y'{
                    print!("{}", Yellow.paint(c.to_string()));
                }
                else if *char_color.get(&c).unwrap() == 'R'{
                    print!("{}", Red.paint(c.to_string()));
                }
            }
            else{
                print!("{}", c);
            }
        }
        print!("\n ");
        for c in second_line.chars(){
            if char_color.contains_key(&c){
                if *char_color.get(&c).unwrap() == 'G'{
                    print!("{}", Green.paint(c.to_string()));
                }
                else if *char_color.get(&c).unwrap() == 'Y'{
                    print!("{}", Yellow.paint(c.to_string()));
                }
                else if *char_color.get(&c).unwrap() == 'R'{
                    print!("{}", Red.paint(c.to_string()));
                }
            }
            else{
                print!("{}", c);
            }
        }
        print!("\n  ");
        for c in third_line.chars(){
            if char_color.contains_key(&c){
                if *char_color.get(&c).unwrap() == 'G'{
                    print!("{}", Green.paint(c.to_string()));
                }
                else if *char_color.get(&c).unwrap() == 'Y'{
                    print!("{}", Yellow.paint(c.to_string()));
                }
                else if *char_color.get(&c).unwrap() == 'R'{
                    print!("{}", Red.paint(c.to_string()));
                }
            }
            else{
                print!("{}", c);
            }
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }


}