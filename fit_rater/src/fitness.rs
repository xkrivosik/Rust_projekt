use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::process::Command;
use std::path::Path;
// Define a struct to represent fitness center information
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
        //Input meno
        println!("Enter name of Fittnes center:");
        name.clear();
        io::stdin().read_line(&mut name).expect("Failed to read name");

        //Input heslo
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


        //Ak meno alebo heslo je prazdne vrati error a skusis zas
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

pub fn display_fitness() {
    // Initialize an empty vector to store fitness center information
    let mut fitness_centers: Vec<FitnessCenter> = Vec::new();

    // Open the file for reading
    if let Ok(lines) = read_lines("src/fittnes_info.txt") {
        // Iterate over lines
        for line in lines {
            if let Ok(ip) = line {
                // Split the line by ':' delimiter
                let parts: Vec<&str> = ip.split(':').collect();

                // Ensure we have enough parts to create a FitnessCenter struct
                if parts.len() == 12 {
                    // Parse the parts into appropriate types
                    let name = parts[0].to_string();
                    let location = parts[1].to_string();
                    let day_price = parts[2].parse().unwrap_or(0);
                    let month_price = parts[3].parse().unwrap_or(0);
                    let year_price = parts[4].parse().unwrap_or(0);
                    let score=0.0;
                    let clean=0;
                    let personal=0;
                    let equip=0;
                    let whole=0;
                    let service=0;
                    let raaters=0;

                    // Create a FitnessCenter instance and push it to the vector
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
    // Now you have all fitness center information in `fitness_centers` vector
    // You can further process or display this information as needed
    let mut id = 0;
    for center in &fitness_centers {
        println!(
"       -------------------------------------------------
           | ID: {}  Name: {}     Location: {}",
            id, center.name, center.location
        );
        id += 1;
    }
    println!("       -------------------------------------------------");
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

    // Open the file for reading
    if let Ok(lines) = read_lines("src/fittnes_info.txt") {
        // Iterate over lines
        for line in lines {
            if let Ok(ip) = line {
                // Split the line by ':' delimiter
                let parts: Vec<&str> = ip.split(':').collect();

                // Ensure we have enough parts to create a FitnessCenter struct
                if parts.len() == 12 {
                    // Parse the parts into appropriate types
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

                    // Create a FitnessCenter instance and push it to the vector
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
    
    display_fitness();
    println!("Enter the ID of the fitness center you want to rate:");
    let mut selected_id = String::new();
    io::stdin().read_line(&mut selected_id).expect("Failed to read input");

    let selected_id: usize = match selected_id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID!");
            return;
        }
    };

    if selected_id >= fitness_centers.len() {
        println!("Invalid ID!");
        return;
    }

    let selected_center = &mut fitness_centers[selected_id];

    // Collect ratings for different aspects
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
    
        // Exit the loop if all ratings are collected successfully
        break;
    }
    // Increase the number of raters for the selected fitness center
    selected_center.raaters += 1;
    selected_center.score=((selected_center.clean*2+selected_center.personal*2+selected_center.equip*3+selected_center.whole*1+selected_center.service*1)as f32)/(selected_center.raaters*9)as f32;
    // Rewrite the text file with updated ratings
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

pub fn inspect() {

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

    display_fitness();

    loop {
    
        println!("Enter the ID of the fitness center you want to inspect:");
        let mut input_id = String::new();
        io::stdin()
            .read_line(&mut input_id)
            .expect("Failed to read input");

       
        let input_id = input_id.trim();

        if input_id.is_empty() {
            continue;
        }

        let index: usize = match input_id.parse() {
            Ok(index) => index,
            Err(_) => {
                println!("Invalid ID!");
                continue;
            }
        };

        if index >= fitness_centers.len() {
            println!("Invalid ID!");
            continue;
        }

        let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
        let selected_fitness = &fitness_centers[index];
        println!(
            "       -------------------------------------------------
            | Name: {}           Day price: {}€    
            | Location: {}       Month price: {}€     
            | ID: {}              Year price: {}€",
            selected_fitness.name,
            selected_fitness.day_price,
            selected_fitness.location,
            selected_fitness.month_price,
            index,
            selected_fitness.year_price
        );
        println!("       -------------------------------------------------");
        break;

    }
    println!("Enter a comment or type 'e' to exit:");
    loop{
    
        let mut comment = String::new();
        io::stdin()
            .read_line(&mut comment)
            .expect("Failed to read input");

        let comment = comment.trim();

        if comment == "e" {
            break;
        }
        else{
            //to do comment insert a save do filu 
            //dal by som ze pri inspect ti vypise komenty aj s menom to staci len aby sa do tejto funkcie posielal curentuser
            //nemam tusenie ako to savnut a inspektnut asi novy txt file a podla indexu
            
        }
    }
}

