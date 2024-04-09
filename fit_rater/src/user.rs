use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

pub fn register() -> String{
    let mut username = String::new();
    let mut password = String::new();

    loop {
        //Input meno
        println!("Enter name:");
        username.clear();
        io::stdin().read_line(&mut username).expect("Failed to read username");

        //Input heslo
        println!("Enter password:");
        password.clear();
        io::stdin().read_line(&mut password).expect("Failed to read password");

        //Ak meno alebo heslo je prazdne vrati error a skusis zas
        if username.trim().is_empty()||password.trim().is_empty(){
            println!("Name or password cannot be empty!");
        }
        else{
            // Ak meno už existuje vyhodí error
            if username_exists(username.trim()) {
                println!("Account with this username already exists.");
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
    writeln!(file, "{}:{}:0", username.trim(), password.trim())
        .expect("Failed to write to file");
    println!("Account {} created",username.trim());

    username
}

pub fn login() -> String{
    let mut username = String::new();
    let mut password = String::new();
    
    loop {
        //Input meno
        println!("Enter name:");
        username.clear();
        io::stdin().read_line(&mut username).expect("Failed to read username");

        //Input heslo
        println!("Enter password:");
        password.clear();
        io::stdin().read_line(&mut password).expect("Failed to read password");

        //Ak meno alebo heslo je prazdne vrati error a skusis zas
        if username.trim().is_empty()||password.trim().is_empty(){
            println!("Name or password cannot be empty!");
        }
        else {
            //Check pre meno a heslo
            if check_credentials(username.trim(), password.trim()) {
                println!("Login successful!\nWelcome {}!",username.trim());
                return username;
            } 
            else {
                println!("Incorrect username or password.");
            }
        }
    }
}

fn username_exists(username: &str) -> bool {
    //Funkcia skenne celuy file ci nenajde meno zo vstupu
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