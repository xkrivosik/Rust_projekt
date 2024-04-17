use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::process::Command;
use std::path::Path;

//fitness center information
#[derive(Debug)]
struct FitnessCenter {
    name: String,
    location: String,
    day_price: i32,
    month_price: i32,
    year_price: i32,
    score: f32,
    clean: i32,
    personal: i32,
    equip: i32,
    whole: i32,
    service: i32,
    raaters: i32,
}

pub fn add_fit(){
    let mut name = String::new();
    let mut location = String::new();
    let mut den_vstup = String::new();
    let mut mes_vstup = String::new();
    let mut rok_vstup = String::new();

    loop {
        println!("Enter name of Fittnes center:");
        name.clear();
        io::stdin().read_line(&mut name).expect("Failed to read name");

        println!("Enter location:");
        location.clear();
        io::stdin().read_line(&mut location).expect("Failed to read location");

        println!("Enter price for 1 day:");
        den_vstup.clear();
        io::stdin().read_line(&mut den_vstup).expect("0");
        println!("Enter price for 1 month:");
        mes_vstup.clear();
        io::stdin().read_line(&mut mes_vstup).expect("0");
        println!("Enter price for 1 year:");
        rok_vstup.clear();
        io::stdin().read_line(&mut rok_vstup).expect("0");

        if name.trim().is_empty()||location.trim().is_empty()||den_vstup.trim().is_empty(){
            println!("Name, location or price is empty");
        }
        
        else{
            break;
        }
    }

    let den_vstup: i32 = den_vstup.trim().parse().expect("Invalid price for 1 day");
    let mes_vstup: i32 = mes_vstup.trim().parse().unwrap_or(0);
    let rok_vstup: i32 = rok_vstup.trim().parse().unwrap_or(0);

    //Inicializacia fileu
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/fittnes_info.txt")
        .expect("Failed to open file");

    //Ulozenie do fileu
    writeln!(file, "{}:{}:{}:{}:{}:0:0:0:0:0:0:0", name.trim(), location.trim(), den_vstup, mes_vstup, rok_vstup)
        .expect("Failed to write to file");
    println!("fitness center: {} is added",name.trim());

    

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn rate_fittnes(){
    let mut fitness_centers: Vec<FitnessCenter> = Vec::new();

    if let Ok(lines) = read_lines("src/fittnes_info.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split(':').collect();

                if parts.len() == 12 {
                    let name = parts[0].to_string();
                    let location = parts[1].to_string();
                    let day_price = parts[2].parse().unwrap_or(0);
                    let month_price = parts[3].parse().unwrap_or(0);
                    let year_price = parts[4].parse().unwrap_or(0);
                    let score = parts[5].parse().unwrap_or(0.0);
                    let clean = parts[6].parse().unwrap_or(0);
                    let personal = parts[7].parse().unwrap_or(0);
                    let equip = parts[8].parse().unwrap_or(0);
                    let whole = parts[9].parse().unwrap_or(0);
                    let service = parts[10].parse().unwrap_or(0);
                    let raaters = parts[11].parse().unwrap_or(0);

                    fitness_centers.push(FitnessCenter {
                        name,
                        location,
                        day_price,
                        month_price,
                        year_price,
                        score,
                        clean,
                        personal,
                        equip,
                        whole,
                        service,
                        raaters,
                    });
                }
            }
        }
    }
    
    let selected_id= inspect() as usize;

    let selected_center = &mut fitness_centers[selected_id];

    loop {
        println!("Enter ratings (0-5) for the following aspects:");
        
        // Cleanliness rating
        println!("Cleanliness:");
        let mut clean_rating = String::new();
        io::stdin().read_line(&mut clean_rating).expect("Failed to read input");
        let clean_rating: i32 = match clean_rating.trim().parse() {
            Ok(rating) if rating >= 0 && rating <= 5 => rating,
            _ => {
                println!("Invalid rating! Please enter a rating between 0 and 5.");
                continue;
            }
        };
        selected_center.clean += clean_rating;
    
        // Personal rating
        println!("Personal:");
        let mut personal_rating = String::new();
        io::stdin().read_line(&mut personal_rating).expect("Failed to read input");
        let personal_rating: i32 = match personal_rating.trim().parse() {
            Ok(rating) if rating >= 0 && rating <= 5 => rating,
            _ => {
                println!("Invalid rating! Please enter a rating between 0 and 5.");
                continue;
            }
        };
        selected_center.personal += personal_rating;
    
        // Equipment quality rating
        println!("Equipment quality:");
        let mut equip_rating = String::new();
        io::stdin().read_line(&mut equip_rating).expect("Failed to read input");
        let equip_rating: i32 = match equip_rating.trim().parse() {
            Ok(rating) if rating >= 0 && rating <= 5 => rating,
            _ => {
                println!("Invalid rating! Please enter a rating between 0 and 5.");
                continue;
            }
        };
        selected_center.equip += equip_rating;
    
        // Overall satisfaction rating
        println!("Overall satisfaction:");
        let mut whole_rating = String::new();
        io::stdin().read_line(&mut whole_rating).expect("Failed to read input");
        let whole_rating: i32 = match whole_rating.trim().parse() {
            Ok(rating) if rating >= 0 && rating <= 5 => rating,
            _ => {
                println!("Invalid rating! Please enter a rating between 0 and 5.");
                continue;
            }
        };
        selected_center.whole += whole_rating;
    
        // Service quality rating
        println!("Service quality:");
        let mut service_rating = String::new();
        io::stdin().read_line(&mut service_rating).expect("Failed to read input");
        let service_rating: i32 = match service_rating.trim().parse() {
            Ok(rating) if rating >= 0 && rating <= 5 => rating,
            _ => {
                println!("Invalid rating! Please enter a rating between 0 and 5.");
                continue;
            }
        };
        selected_center.service += service_rating;
    
        break;
    }
    selected_center.raaters += 1;
    selected_center.score=((selected_center.clean*2+selected_center.personal*2+selected_center.equip*3+selected_center.whole*1+selected_center.service*1)as f32)/(selected_center.raaters*9)as f32;

    if let Ok(mut file) = File::create("src/fittnes_info.txt") {
        for center in &fitness_centers {
            writeln!(
                file,
                "{}:{}:{}:{}:{}:{:.2}:{}:{}:{}:{}:{}:{}",
                center.name,
                center.location,
                center.day_price,
                center.month_price,
                center.year_price,
                center.score,
                center.clean,
                center.personal,
                center.equip,
                center.whole,
                center.service,
                center.raaters
            ).expect("Failed to write to file");
        }
    } else {
        println!("Failed to open file for writing!");
    }
}

pub fn inspect()->usize {
    let mut fitness_centers: Vec<FitnessCenter> = Vec::new();

    if let Ok(lines) = read_lines("src/fittnes_info.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split(':').collect();

                if parts.len() == 12 {
                    let name = parts[0].to_string();
                    let location = parts[1].to_string();
                    let day_price = parts[2].parse().unwrap_or(0);
                    let month_price = parts[3].parse().unwrap_or(0);
                    let year_price = parts[4].parse().unwrap_or(0);

                    let fitness_center = FitnessCenter {
                        name,
                        location,
                        day_price,
                        month_price,
                        year_price,
                        score: 0.0,
                        clean: 0,
                        personal: 0,
                        equip: 0,
                        whole: 0,
                        service: 0,
                        raaters: 0,
                    };
                    fitness_centers.push(fitness_center);
                }
            }
        }
    }

    let index;
    let mut display_index = 0;
    let mut _exit = false;

    loop {
        let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

        if !output.success() {
            eprintln!("Failed to clear terminal");
        }
        display_fitnessi(&fitness_centers, display_index);

        println!("Enter the ID of the fitness center you want to inspect:\n'n' -> next page\n'p' -> previous page");
        let mut input_id = String::new();
        io::stdin()
            .read_line(&mut input_id)
            .expect("Failed to read input");

        let input_id = input_id.trim();

        if input_id == "e" {
            break;
        }

        match input_id {
            "p" => {
                if display_index >= 10 {
                    display_index -= 10;
                } else {
                    println!("You're already at the first page.");
                }
            }
            "n" => {
                if display_index < fitness_centers.len() - 10 {
                    display_index += 10;
                } else {
                    let remaining = fitness_centers.len() - display_index - 1;
                    println!("You're already at the last page with {} records.", remaining);
                }
            }
            _ => {
                if let Ok(parsed_index) = input_id.parse::<usize>() {
                    if parsed_index < fitness_centers.len() {
                        index = parsed_index;
                        display_index = index;
                    } else {
                        println!("Invalid ID!");
                    }
                    
                    break;
                } else {
                    println!("Invalid input!");
                }
            }
        }
    }
    return display_index  ;

}

fn display_fitnessi(fitness_centers: &Vec<FitnessCenter>, start_index: usize) {
    const PAGE_SIZE: usize = 10;

    println!("Fitness Centers:");
    for (i, fitness_center) in fitness_centers.iter().enumerate().skip(start_index).take(PAGE_SIZE) {
        println!("ID: {} - Name: {}", i, fitness_center.name);
    }
}

pub fn inspection(username:&String) {
    let mut fitness_centers: Vec<FitnessCenter> = Vec::new();
    
    if let Ok(lines) = read_lines("src/fittnes_info.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split(':').collect();

                if parts.len() == 12 {
                    let name = parts[0].to_string();
                    let location = parts[1].to_string();
                    let day_price = parts[2].parse().unwrap_or(0);
                    let month_price = parts[3].parse().unwrap_or(0);
                    let year_price = parts[4].parse().unwrap_or(0);
                    let score = parts[5].parse().unwrap_or(0.0);
                    let clean = 0;
                    let personal = 0;
                    let equip = 0;
                    let whole = 0;
                    let service = 0;
                    let raaters = 0;

                    fitness_centers.push(FitnessCenter {
                        name,
                        location,
                        day_price,
                        month_price,
                        year_price,
                        score,
                        clean,
                        personal,
                        equip,
                        whole,
                        service,
                        raaters,
                    });
                }
            }
        }
    }
    let  index=inspect();
    let mut back = String::new();
    loop {
        let output = Command::new("cmd")
                 .args(&["/C", "cls"])
                 .status()
                 .expect("Failed to clear terminal");

                 if !output.success() {
                      eprintln!("Failed to clear terminal");
                             }
        if index < fitness_centers.len() {
                println!("-------------------------------------------------");
                println!("| ID: {}  Name: {}  Location: {}", index, fitness_centers[index].name, fitness_centers[index].location);
                println!("| Rating: {}/5({})",fitness_centers[index].score,fitness_centers[index].raaters);
                println!("-------------------------------------------------");
        } else {
                println!("Index out of bounds!");
                }
                display_coment(index);
                
        println!("\nPress 'e' to go back or 'c' to comment fitness center");
        back.clear();
        io::stdin().read_line(&mut back).expect("Failed to read command.");

        
        if back.trim()=="e"{
        break;
        }
        else if back.trim()=="c"{
            comment(username,index);
        }
        
    }
     
}

fn comment(username:&String,index:usize){
    println!("Coment:");
    let mut coment= String::new();
    coment.clear();
    io::stdin().read_line(&mut coment).expect("Failed to comment.");
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("src/comment.txt")
    .expect("Failed to open file");

//Ulozenie do fileu
writeln!(file, "{}|{}:{}", index, username.trim(), coment.trim())
    .expect("Failed to write to file");
println!("comment added");

}
 struct ComentLog {
    id: usize,
    comment: String,
}

fn display_coment(index: usize) {
    let mut comments: Vec<ComentLog> = Vec::new();

    if let Ok(lines) = read_lines("src/comment.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split('|').collect();

                if parts.len() == 2 {
                    let id = parts[0].parse::<usize>().unwrap_or(0);
                    let comment = parts[1].to_string();

                    comments.push(ComentLog {
                        id,
                        comment,
                    });
                }
            }
        }
    }

    for comment in &comments {
        if comment.id == index {
            println!("{}", comment.comment);
        }
    }
   
}
