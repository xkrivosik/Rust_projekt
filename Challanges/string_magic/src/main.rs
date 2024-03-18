use std::io;

fn main() {
    let mut input_string = String::new();
    let mut input_choice = String::new();

    println!("Enter a string:");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    println!("What do you wandt to do?\n-> l - lowercase\n-> u - uppercase\n-> r - reversed");
    io::stdin().read_line(&mut input_choice).expect("Failed to read line");
    let choice:char = input_choice.chars().next().unwrap();

    if choice == 'l'{
        let lowercase = input_string.to_lowercase();
        println!("lowercased: {}",lowercase);
    }
    if choice == 'u'{
        let uppercase = input_string.to_uppercase();
        println!("uppercased: {}",uppercase);
    }
    if choice == 'r'{
        let reversed: String = input_string.chars().rev().collect();
        println!("reversed: {}",reversed);
    }
}
