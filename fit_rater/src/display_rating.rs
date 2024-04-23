use std::fs::File;
use std::io::{self, BufRead};

pub struct Person {
    name: String,
    value: i32,
}

impl Person {
    fn new(name: &str, value: i32) -> Self {
        Person {
            name: name.to_string(),
            value,
        }
    }
}

pub fn top_rates() {
    // Open the file
    if let Ok(file) = File::open("src/user_info.txt") {
        let mut people: Vec<Person> = Vec::new();

        // Read the file line by line
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                // Split each line into name and value
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 3 {
                    if let Ok(value) = parts[2].trim().parse::<i32>() {
                        let person = Person::new(parts[0].trim(), value);
                        people.push(person);
                    }
                }
            }
        }

        // Bubble sort the vector in descending order based on the value field
        for i in 0..people.len() {
            for j in 0..people.len() - 1 - i {
                if people[j].value < people[j + 1].value {
                    people.swap(j, j + 1);
                }
            }
        }

        // Print the sorted vector
        println!("Sorted by value in descending order:");
        for (index, person) in people.iter().take(5).enumerate() {
            println!("{}. {} : {}", index + 1, person.name, person.value);
        }
    } else {
        println!("Failed to open user_info.txt");
    }
}
pub fn top_fit(){
    
}


