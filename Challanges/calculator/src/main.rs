use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number, a mathematical symbol, and another number:");

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

    // Parsing the mathematical symbol
    let symbol: char = match components.next() {
        Some(sym) => sym.chars().next().unwrap(),
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
    if symbol == '+'{
        println!("{}",number1+number2);
    }
    if symbol == '-'{
        println!("{}",number1-number2);
    }
    if symbol == '*'{
        println!("{}",number1*number2);
    }
    if symbol == '/'{
        println!("{}",number1/number2);
    }
}
