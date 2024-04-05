use std::fs::OpenOptions;
use std::io::{self, Write, BufRead, BufReader};

// Function to check if the username already exists in the file
fn username_exists(username: &str) -> bool {
    let file = OpenOptions::new()
        .read(true)
        .open("src/login.txt")
        .expect("Failed to open file");

    let reader = BufReader::new(file);

    reader.lines().any(|line| {
        if let Ok(line) = line {
            line.starts_with(&format!("{}||", username.trim()))
        } else {
            false
        }
    })
}

pub fn new_acc() {
    let mut username = String::new();
    let mut password = String::new();

    loop {
        println!("Zadaj meno:");
        username.clear();
        io::stdin().read_line(&mut username).expect("Failed to read username");

        println!("Zadaj Heslo:");
        password.clear();
        io::stdin().read_line(&mut password).expect("Failed to read password");

        if username.trim().is_empty() || password.trim().is_empty() {
            println!("invalid input!!!");
            continue;
        }

        if username_exists(&username) {
            println!("name already used");
            continue;
        } else {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("src/login.txt")
                .expect("Failed to open file");
            writeln!(file, "{}||{}", username.trim(), password.trim())
                .expect("Failed to write to file");
            println!("Username and password saved");
            break;
        }
    }
}
