use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn lgoin(){
    let mut username = String::new();
    let mut password = String::new();
    loop {
    println!("Zadaj meno:");
    username.clear();
    io::stdin().read_line(&mut username).expect("Failed to read username");

    println!("Zadaj Heslo:");
    password.clear();
    io::stdin().read_line(&mut password).expect("Failed to read password");
   if username.trim().is_empty()||password.trim().is_empty(){
        println!("invalid input!!!");
    }
    else{
        break;
    }
}
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/login.txt")
        .expect("Failed to open file");
    //comand na print to file 
    writeln!(file, "{}||{}", username.trim(), password.trim())
        .expect("Failed to write to file");
    println!("Username and password saved");
}