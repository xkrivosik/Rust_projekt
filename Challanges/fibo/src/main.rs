use std::io;

fn main() {
    let mut prva=0;
    let mut druha=1;
    let mut tretia=0;
    let mut pomocna;
    let mut pocet=0;

    let mut input = String::new();
    println!("Enter two numbers devided by a space:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Splitting input into components
    let mut components = input.trim().split_whitespace();

    // Parsing the first number
    let number1: i32 = match components.next() {
        Some(num_str) => match num_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the first number.");
                return;
            }
        },
        None => {
            println!("Invalid input format.");
            return;
        }
    };

    // Parsing the second number
    let number2: i32 = match components.next() {
        Some(num_str) => match num_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the second number.");
                return;
            }
        },
        None => {
            println!("Invalid input format.");
            return;
        }
    };

    while prva<=number2{
        if prva>=number1 && prva<=number2{
            pocet+=1;
        }
        prva=druha+tretia;
        pomocna=prva;
        tretia=druha;
        druha=pomocna;
    }
    println!("{}",pocet+1);
}
