pub mod tty_mode{
    use std::{collections::HashMap, fs, io::{self, Write}};
    use ansi_term::Colour::{Green,Red,Yellow};
    use std::path::Path;
    use serde_json::Error as SerdeError;
    use crate::{get_useable_word_default, get_useable_word_file, read_lines_from_file, select, state_to_json, Config, State, convert_keys_to_uppercase, hash_map_sort, judge::crate_judge::judge_tty};

    pub fn tty() -> Result<(), Box<dyn std::error::Error>>{
        let default_config = Config{
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
        let mut name = String::new();
        let output = r"
 __        __            _ _      
 \ \      / /__  _ __ __| | | ___ 
  \ \ /\ / / _ \| '__/ _` | |/ _ \
   \ V  V / (_) | | | (_| | |  __/
    \_/\_/ \___/|_|  \__,_|_|\___|                             ";
        print!("{}\n", output);
        io::stdout().flush().unwrap();
        let mut config: Config;
        match json_to_config_tty("src/data/config.json") {
            Ok(config_in) => {
                println!("Configuration loaded config {}",Green.paint("Successfully"));
                config = config_in;
            }
            Err(e) => {
                let red_color = "\x1B[31m";
                let reset_color = "\x1B[0m";
                if e.to_string() == "File has been moved" {
                    println!("{}Error: File has been moved{}", red_color, reset_color);
                } else if e.is::<SerdeError>() {
                    println!("{}Error: File is corrupted{}", red_color, reset_color);
                } else {
                    println!("{}Error: {}{}", red_color, e, reset_color);
                }
                config = default_config;
            }
        }
        println!("Welcome to Wordle!");
        println!("This is tty mode.");
        println!("Please enter your name: ");
        io::stdin().read_line(&mut name).unwrap();
        println!("Hello,{}Press enter to start the game.If you need any help, please enter --help to get help message.If you want to change the Setting , please enter --set." , name);
        let mut input = String::new();
        loop{
            io::stdin().read_line(&mut input).unwrap();
            println!("\n\n");
            if input.trim() == "--help"{
                help_msg();
            }
            else if input.trim() == "--set"{
                config = setconfig();
            }else if input.trim() == ""{
                let _ = game(config.clone());
                break;
            }else{
                println!("Please enter the correct command.");
            }
            input.clear();
        }
        Ok(())
    }

    fn help_msg(){
        println!("Welcome the help message!");
            println!("Wordle is a game that you need to guess the word.");
            println!{"You can enter a five-letter word to guess the word.\n"};
            println!{"If the letter is in the word and in the right position, it will be marked in {}." , Green.paint("Green")};
            println!{"If the letter is in the word but not in the right position, it will be marked in {}." , Yellow.paint("Yellow")};
            println!{"If the letter is not in the word, it will be marked in {}.\n" , Red.paint("Red")};
            println!{"Also , you can see a alphabet set which shows the best result of letters you have guessed."};
            println!("Please correctly use the hits to guess the word!\n");
            println!("Here are the commands you can use:");
            println!("--help: get help message");
            println!("--random: generate a random word with a day and a seed.");
            println!("--difficult: set the game to difficult mode.which means you can't change the {} letter,and you will not permissed to not use the {} letter." , Red.paint("Red") , Yellow.paint("Yellow"));
            println!("--stats: show the statistics of the game");
            println!("--day: set the day of the game");
            println!("--seed: set the seed of random of the game");
            println!("--final_set: set the final set path of the game");
            println!("--acceptable_set: set the acceptable set path of the game");
            println!("--state: set the state path of the game");
            println!("--word: set the word of the game");
    }

    fn game(mut config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let mut used_word_frequency = HashMap::new();
        let mut answer = String::new();
        let mut count_played = 0;
        let mut count_success = 0;
        let mut count_success_loop = 0;
        let mut final_set = Vec::new();
        let mut acceptable_set = Vec::new();
        let mut temp1 = String::new();
        let mut temp2 = String::new();

        if &config.final_set.clone().unwrap() != ""{
            final_set = read_lines_from_file(&config.final_set.clone().unwrap(), &mut temp1).unwrap();
        }
        else{
            final_set = select::FINAL.to_vec();
        }

        if &config.acceptable_set.clone().unwrap() != ""{
            acceptable_set = read_lines_from_file(&config.acceptable_set.clone().unwrap(), &mut temp2).unwrap();
        }
        else{
            acceptable_set = select::ACCEPTABLE.to_vec();
        }
        
        let mut json = State{
            total_rounds : Some(0),
            games : Some(Vec::new()),
        };
        if &config.state.clone().unwrap() != ""{
            let data = fs::read_to_string(&config.state.clone().unwrap()).unwrap();
            json = serde_json::from_str(&data).unwrap();
        }
        if !crate::is_subset(&final_set, &acceptable_set){
            println!("{}" , Red.paint("Error : INVALID"));
            panic!("The final set is not a subset of the acceptable set.");
        }
        let random = config.random.clone();
        let mut _flag = true;
        if random.unwrap(){
            let mut answer_used = Vec::new();
            loop{
                let mut line = String::new();
                if &config.final_set.clone().unwrap() != ""{
                    line = get_useable_word_file(config.day.unwrap(), config.seed.unwrap().try_into().unwrap(), config.final_set.clone().unwrap().as_str());
                }
                else{
                    line = get_useable_word_default(config.day.unwrap(), config.seed.unwrap().try_into().unwrap());
                }
                if answer_used.contains(&line){                     //Check if the word has been used
                    continue;
                }
            
                let (success , gusses , frequency) = judge_tty(&line , config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
                used_word_frequency = frequency;
                _flag = false;
                answer = line;
                answer_used.push(answer.clone());
                count_played += 1;
                let state = config.state.clone().unwrap();
                if success != 0{
                    count_success += 1;
                    count_success_loop += success;            
                    }
                if config.stats.unwrap(){
                    print_state_tty(count_success , count_played , count_success_loop , used_word_frequency.clone() , state.clone() != "" , json.clone());
                }
                if &config.state.clone().unwrap() != ""{
                    let _ = state_to_json(state.clone() , answer.clone() , gusses.clone());
                }

                success_judge_tty(false , success, answer.clone());
                    
                if count_played != 0{                 //Check if player want another round
                    let mut _flag = true;
                    let mut line = String::new();
                    io::stdin().read_line(&mut line).unwrap();
                    if line == "N\n" || line == "n\n" || line == "N" || line == "n"{
                        return Ok(());
                    }
                }

                config.day = Some(config.day.unwrap() + 1);
            }
        }
        let mut word = config.word.clone().unwrap();
        if word != "" && _flag{
            loop{
                if !acceptable_set.contains(&word.as_str()){
                    println!("{}" , Red.paint("Error : INVALID INPUT"));
                    println!("If you want to change the word, please enter it or N/n to exit.");
                    io::stdin().read_line(&mut word).unwrap();
                    word.pop();
                    if word == "N" || word == "n"{
                        return Ok(());
                    }
                    else{
                        continue;
                    }
                }
                else{
                    break;
                }
            }
            let (success , gusses , frequency) = judge_tty(&word , config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
            used_word_frequency = frequency;
            answer = word;
            count_played += 1;
            let state = config.state.clone().unwrap();
            _flag = false;
            if success != 0{
                count_success += 1;
                count_success_loop += success;            
            }
            if config.stats.unwrap(){
                print_state_tty(count_success , count_played , count_success_loop , used_word_frequency.clone() , state.clone() != "" , json.clone());
            }
            if &config.state.clone().unwrap() != "" {
                let _ = state_to_json(state.clone() , answer.clone() , gusses.clone());
            }
            success_judge_tty(true , success, answer.clone());
        }
        else{
            println!("Please enter the word you want to guess.");
            let mut word = String::new();
            io::stdin().read_line(&mut word).unwrap();
            word.pop();
            if !acceptable_set.contains(&word.as_str()){
                println!("{}" , Red.paint("Error : INVALID INPUT"));
                println!("If you want to change the word, please enter it enter N/n to exit.");
            }
            let (success , gusses , frequency) = judge_tty(&word , config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
            used_word_frequency = frequency;
            answer = word;
            count_played += 1;
            let state = config.state.clone().unwrap();
            _flag = false;
            if success != 0{
                count_success += 1;
                count_success_loop += success;            
            }
            if config.stats.unwrap(){
                print_state_tty(count_success , count_played , count_success_loop , used_word_frequency.clone() , state.clone() != "" , json.clone());
            }
            if &config.state.clone().unwrap() != ""{
                let _ = state_to_json(state.clone() , answer.clone() , gusses.clone());
            }
            success_judge_tty(true , success, answer.clone());
        }

        Ok(())
    }

    fn setconfig()-> Config{
        println!("Tips:Please press Enter if you want to keep the default config.");
        println!("Please enter y in the bool question if you wang the specific mode.");
        let mut input = String::new();
        print!("random mode: ");io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut config = Config{
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
        loop{
            if input.trim() == "y" || input.trim() == "Y"{
                config.random = Some(true);      
            }
            else if input.trim() == ""{
                config.random = Some(false);
            }
            else{
                println!("We can't regonginze your command :)");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                continue;
            }
            if input.trim() == "y" || input.trim() == "Y" || input.trim() == ""{
                input.clear();
                break;
            }
        }
        loop{
            print!("difficult mode: ");io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" || input.trim() == "Y"{
                config.difficult = Some(true);      
            }
            else if input.trim() == ""{
                config.difficult = Some(false);
            }
            else{
                println!("We can't regonginze your command :)");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                continue;
            }
            if input.trim() == "y" || input.trim() == "Y" || input.trim() == ""{
                input.clear();
                break;
            }
        }
        loop{
            print!("stats mode: ");io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "Y" || input.trim() == "y"{
                config.stats = Some(true);      
            }
            else if input.trim() == ""{
                config.stats = Some(false);
            }
            else{
                println!("We can't regonginze your command :)");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                continue;
            }
            if input.trim() == "Y" || input.trim() == "y" || input.trim() == ""{
                input.clear();
                break;
            }
        }
        loop {
            input.clear();
            print!("day: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().is_empty() {
                config.day = Some(1);
                break;
            } else {
                match input.trim().parse::<usize>() {
                    Ok(day) => {
                        config.day = Some(day);
                        break;
                    }
                    Err(_) => {
                        println!("{}. Please enter a valid number.",Red.paint("Error : INVALID INPUT"));
                    }
                }
            }
        }
        loop {
            input.clear();
            print!("seed: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().is_empty() {
                config.seed = Some(1);
                break;
            } else {
                match input.trim().parse::<i32>() {
                    Ok(seed) => {
                        config.seed = Some(seed);
                        break;
                    }
                    Err(_) => {
                        println!("{}. Please enter a valid number.",Red.paint("Error : INVALID INPUT"));
                    }
                }
            }
        }
        loop {
            input.clear();
            print!("final_set: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                config.final_set = Some("".to_string());
                break;
            } else {
                if Path::new(trimmed_input).exists() {
                    config.final_set = Some(trimmed_input.to_string());
                    break;
                } else {
                    println!("{}. Please enter a valid path." , Red.paint("Error : INVALID PATH"));
                }
            }
        }
        loop {
            input.clear();
            print!("acceptable_set: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                config.acceptable_set = Some("".to_string());
                break;
            } else {
                if Path::new(trimmed_input).exists() {
                    config.acceptable_set = Some(trimmed_input.to_string());
                    break;
                } else {
                    println!("{}. Please enter a valid path." , Red.paint("Error : INVALID PATH"));
                }
            }
        }
        loop {
            input.clear();
            print!("state: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                config.state = Some("".to_string());
                break;
            } else {
                if Path::new(trimmed_input).exists() {
                    config.state = Some(trimmed_input.to_string());
                    break;
                } else {
                    println!("{}. Please enter a valid path." , Red.paint("Error : INVALID PATH"));
                }
            }
        }
        loop{
            print!("word:");io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == ""{
                config.word = Some("".to_string());      
            }
            else{
                config.word = Some(input.trim().to_string());
            }
            if input.trim() == ""{
                break;
            }
        }
        config
    }

    fn success_judge_tty(_flag:bool , success : i32 , answer : String){
        if success != 0{
            println!("Congratulations! You have guessed the word {}" , answer.to_uppercase());
            println!("If you are in the Random mode ,you can choose to continue or not.");
            println!("Enter Y / y to continue another round......");
        }
        else {
            println!("残念です。The answer is {} ." , answer.to_uppercase());
            println!("But you can choose to start another round 。 Fight!");
            println!("Enter Y / y to continue another round......");
        }
    }

    fn print_state_tty(mut count_success: i32, mut count_played: i32, mut count_success_loop: i32, mut used_word_frequency: HashMap<String, i32> , flag : bool , json : State) {
    
        used_word_frequency = convert_keys_to_uppercase(used_word_frequency);
        if flag {
            if let Some(games) = json.games {
                count_played += games.len() as i32;
                for game in games {
                    if let Some(guesses) = game.guesses {
                        count_success_loop += guesses.len() as i32;
                        count_success += if game.answer.is_some() { 1 } else { 0 };
                        for guess in guesses {
                            let counter = used_word_frequency.entry(guess.clone()).or_insert(0);
                            *counter += 1;
                        }
                    }
                }
            }
        }
        if count_success != 0 {
            let success_rate = count_success_loop as f64 / count_success as f64;
            println!("You have succeed for {} rounds" , count_success);
            println!("You have played for {} rounds" , count_played);
            println!("The average success round is {:.2}" , success_rate);
        } else {
            println!("You have played for {} rounds" , count_played);
            println!("You have not succeed for any round");
            println!("ざこざこ～～")
        }
    
        let mut vec = hash_map_sort(used_word_frequency);
        vec = vec.iter().take(5).cloned().collect();
        println!("The top 5 words you have used are:");
        for (word, count) in vec {
            println!("You have used the word {} for {} times" , word , count);
        }
    }

    fn json_to_config_tty(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        // Check if the file exists
        if !path.exists() {
            return Err(Box::from("File has been moved"));
        }  
        // Open the file
        let data = fs::read_to_string(&path).unwrap();
        // Parse the JSON
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }
}

