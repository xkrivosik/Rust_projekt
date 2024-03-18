use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess a number 1-10");

    let mut input = String::new();
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=10);

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };

    if number == random_number {
        println!("Yaaay you guess right :)");
    }
    if number != random_number {
        println!("Too bad you guessed wrong :(");
    }
    println!("Your guess: {} Random number: {}",number,random_number);
}
