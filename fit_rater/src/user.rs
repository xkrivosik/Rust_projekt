use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{exit, Command};

pub fn register() -> String{
    let mut username = String::new();
    let mut password = String::new();

    loop {
        //Input meno
        println!("To exit type 'e' into the name\nEnter name:");
        username.clear();
        io::stdin().read_line(&mut username).expect("Failed to read username");

        if username.trim() == "e"{
            exit(0);
        }

        //Input heslo
        println!("Enter password:");
        password.clear();
        io::stdin().read_line(&mut password).expect("Failed to read password");

        //Ak meno alebo heslo je prazdne vrati error a skusis zas
        if username.trim().is_empty()||password.trim().is_empty(){
            let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
            println!("Name or password cannot be empty!");
        }
        else{
            // Ak meno už existuje vyhodí error
            if username_exists(username.trim()) {
                let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
                println!("Account with username '{}' already exists.",username.trim());
            } 
            else {
                break;
            }
        }
    }

    //Inicializacia fileu
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/user_info.txt")
        .expect("Failed to open file");

    //Ulozenie do fileu
    let encrpted_pass = encrypt(password.trim());
    writeln!(file, "{}:{}:0", username.trim(), encrpted_pass.trim())
        .expect("Failed to write to file");
    println!("Account {} created",username.trim());

    username
}

pub fn login() -> String{
    let mut username = String::new();
    let mut password = String::new();
    
    loop {
        //Input meno
        println!("To exit type 'e' into the name\nEnter name:");
        username.clear();
        io::stdin().read_line(&mut username).expect("Failed to read username");

        if username.trim() == "e"{
            exit(0);
        }
        //Input heslo
        println!("Enter password:");
        password.clear();
        io::stdin().read_line(&mut password).expect("Failed to read password");

        //Ak meno alebo heslo je prazdne vrati error a skusis zas
        if username.trim().is_empty()||password.trim().is_empty(){
            let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
            println!("Name or password cannot be empty!");
        }
        else {
            //Check pre meno a heslo
            let decrpyt_pass = encrypt(password.trim());
            if check_credentials(username.trim(), decrpyt_pass.trim()) {
                println!("Login successful!\nWelcome {}!",username.trim());
                return username;
            } 
            else {
                let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
                println!("Incorrect username or password.");
            }
        }
    }
}

fn username_exists(username: &str) -> bool {
    //Funkcia skenne cely file ci nenajde meno zo vstupu
    if let Ok(file) = File::open("src/user_info.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(name) = parts.get(0) {
                    if *name == username {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn check_credentials(username: &str, password: &str) -> bool {
    //Funkcia hlada meno:heslo zo vstupu vo file
    if let Ok(file) = File::open("src/user_info.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(':').collect();
                if let (Some(name), Some(pass)) = (parts.get(0), parts.get(1)) {
                    if *name == username && *pass == password {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn get_score(username: &str) -> i32 {
    //Najde skore pre zadaneho usera
    if let Ok(file) = File::open("src/user_info.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(':').collect();
                if let (Some(name), Some(score_str)) = (parts.get(0), parts.get(2)) {
                    if *name == username {
                        if let Ok(score) = score_str.trim().parse::<i32>() {
                            return score;
                        } else {
                            println!("Invalid score format for user: {}", username);
                            return -1;
                        }
                    }
                }
            }
        }
    }
    println!("User {} not found", username);
    -1
}

fn encrypt(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    for ch in input.chars() {
        let new_char = match ch {
            ' ' => ' ',
            _ => {
                let unicode_value = ch as u32;
                let mut new_unicode_value = unicode_value + match i {
                    1 => 9,
                    2 => 6,
                    3 => 4,
                    4 => 7,
                    5 => 5,
                    6 => 13,
                    7 => 2,
                    8 => 11,
                    9 => 3,
                    10 => 15,
                    11 => 3,
                    12 => 14,
                    13 => 9,
                    14 => 12,
                    15 => 4,
                    _ => 1,
                };
                // Adjust the new unicode value if it results in ':'
                if new_unicode_value == ':' as u32 {
                    new_unicode_value += 1;
                }
                i += 1;
                std::char::from_u32(new_unicode_value).unwrap()
            }
        };
        result.push(new_char);
    }
    result
}

pub fn get_rank(score:i32) ->String{
    if score>=50 && score<250{
        return "Advanced Rater".to_string();
    }
    if score>=250 && score<500{
        return "Pro Rater".to_string();
    }
    if score>=500{
        return "Master Rater".to_string();
    }
    else{
        return "Novice Rater".to_string();
    }
}
struct User {
    name: String,
    pass: String,
    score: i32,
}

pub fn score_update(username: String) {
    let mut users: Vec<User> = Vec::new();
    println!("{}",username);
    if let Ok(file) = File::open("src/user_info.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(user_info) = line {
                let parts: Vec<&str> = user_info.split(':').collect();
                if parts.len() == 3 {
                    let name = parts[0].trim().to_string();
                    let pass = parts[1].trim().to_string();
                    let score: i32 = parts[2].trim().parse().unwrap_or(0);

                    let user = User { name, score, pass };
                    users.push(user);
                }
            }
        }
    } else {
        println!("Failed to open the file.");
        return;
    }

    if let Some(user) = users.iter_mut().find(|u| u.name == username) {
        user.score += 1;
        println!("Score updated for user {}.", username);
    } else {
        println!("User {} not found.", username);
        return;
    }

    if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open("src/user_info.txt") {
        for user in &users {
            writeln!(file, "{}:{}:{}", user.name, user.pass, user.score).expect("Failed to write to file");
        }
       
    } else {
        println!("Failed to open the file for writing.");
    }
}
