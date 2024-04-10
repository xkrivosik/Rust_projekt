use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

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
            let decrpyt_pass = encrypt(password.trim());
            if check_credentials(username.trim(), decrpyt_pass.trim()) {
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

/*poriesil som tu dvojbodku v encrptovani ak pouzijeme aj toto budeme musiet to poriesit aj tu
fn decrypt(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    for ch in input.chars() {
        let new_char = match ch {
            ' ' => ' ',
            _ => {
                let unicode_value = ch as u32;
                let new_unicode_value = unicode_value - match i {
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
                i += 1;
                std::char::from_u32(new_unicode_value).unwrap()
            }
        };
        result.push(new_char);
    }
    result
}*/

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
/*Nefunguje */
/*pub fn update_score(name: &str, score: i32) {
    // Open the file in read/write mode
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("src/user_info.txt");

    // Handle errors opening the file
    let file = match file {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file.");
            return;
        }
    };

    // Create a buffered reader to read lines from the file
    let reader = BufReader::new(&file);

    // Create a vector to hold the updated lines
    let mut updated_lines = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Failed to read line.");
                return;
            }
        };

        // Split the line into parts separated by ':'
        let parts: Vec<&str> = line.split(':').collect();

        // Check if the line corresponds to the user we're looking for
        if let Some((user_score_str, rest)) = parts.split_last() {
            if let Ok(user_score) = user_score_str.parse::<i32>() {
                // If this is the user we're looking for, update the score
                if parts.get(0) == Some(&name) {
                    let updated_score = user_score + score;
                    let updated_line = format!("{}:{}:{}", name, rest.join(":"), updated_score);
                    updated_lines.push(updated_line);
                    continue; // Skip writing the original line
                }
            }
        }

        // If the line doesn't correspond to the user we're looking for, add it unchanged to the vector
        updated_lines.push(line);
    }

    // Reopen the file in write mode to clear its contents
    let mut file = match OpenOptions::new().write(true).truncate(true).open("src/user_info.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file for writing.");
            return;
        }
    };

    // Write the updated lines back to the file
    for line in updated_lines {
        if let Err(_) = writeln!(file, "{}", line) {
            eprintln!("Failed to write line to file.");
            return;
        }
    }
}*/
struct User {
    name: String,
    pass: String,
    score: i32,
}

pub fn score_update(username: String) {
    // Create a vector to store user data
    let mut users: Vec<User> = Vec::new();
    println!("{}",username);
    // Open the file for reading
    if let Ok(file) = File::open("src/user_info.txt") {
        let reader = BufReader::new(file);

        // Iterate over lines in the file
        for line in reader.lines() {
            if let Ok(user_info) = line {
                // Split the line by ':' delimiter
                let parts: Vec<&str> = user_info.split(':').collect();
                if parts.len() == 3 {
                    // Parse user name, password, and score
                    let name = parts[0].trim().to_string();
                    let pass = parts[1].trim().to_string();
                    let score: i32 = parts[2].trim().parse().unwrap_or(0);

                    // Create a User instance and push it to the vector
                    let user = User { name, score, pass };
                    users.push(user);
                }
            }
        }
    } else {
        println!("Failed to open the file.");
        return;
    }

    // Find the user with the specified username
    if let Some(user) = users.iter_mut().find(|u| u.name == username) {
        user.score += 1;
        println!("Score updated for user {}.", username);
    } else {
        println!("User {} not found.", username);
        return;
    }

    // Open the file for writing
    if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open("src/user_info.txt") {
        // Write updated user information back to the file
        for user in &users {
            writeln!(file, "{}:{}:{}", user.name, user.pass, user.score).expect("Failed to write to file");
        }
       
    } else {
        println!("Failed to open the file for writing.");
    }
}
