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
pub struct Fitness {
    name: String,
    location: String,
    score: f32,
    number:i32,
}

impl Fitness {
    fn new(name: &str, location: &str, score: f32,number: i32) -> Self {
        Fitness {
            name: name.to_string(),
            location: location.to_string(),
            score,
            number,
        }
    }
}

pub fn top_rates() {
    if let Ok(file) = File::open("src/user_info.txt") {
        let mut people: Vec<Person> = Vec::new();

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 3 {
                    if let Ok(value) = parts[2].trim().parse::<i32>() {
                        let person = Person::new(parts[0].trim(), value);
                        people.push(person);
                    }
                }
            }
        }

        for i in 0..people.len() {
            for j in 0..people.len() - 1 - i {
                if people[j].value < people[j + 1].value {
                    people.swap(j, j + 1);
                }
            }
        }

        for (index, person) in people.iter().take(5).enumerate() {
            println!("{}. {} : {}", index + 1, person.name, person.value);
        }
    } else {
        println!("Failed to open user_info.txt");
    }
}

pub fn top_fitness() {
    if let Ok(file) = File::open("src/fittnes_info.txt") {
        let mut fitness_centers: Vec<Fitness> = Vec::new();

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 12 {
                    match (
                        parts[5].trim().parse::<f32>(),
                        parts[0].trim().parse::<String>(),
                        parts[1].trim().parse::<String>(),
                        parts[11].trim().parse::<i32>(),
                    ) {
                        (Ok(score), Ok(name), Ok(location), Ok(number)) => {
                            let fitness_center = Fitness::new(&name, &location, score, number);
                            fitness_centers.push(fitness_center);
                        }
                        _ => {
                            println!("Failed to parse score, name, location, or number");
                        }
                    }
                } else {
                    println!("Invalid number of parts in the line");
                }
            } else {
                println!("Failed to read line");
            }
        }
        
        fitness_centers.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        println!("Top 5 Fitness Centers:\n");
        for (index, fitness) in fitness_centers.iter().take(5).enumerate() {
            println!(
                "{}. {}       {} 
score: {:.2}       rates: {}\n",
                index + 1,
                fitness.name,
                fitness.location,
                fitness.score,
                fitness.number
            );
        }
    } else {
        println!("Failed to open fittnes_info.txt");
    }
}