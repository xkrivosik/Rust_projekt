use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
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
    let mut id=0;
    for center in &fitness_centers {
        println!("      ----------------------------------------
       | Name: {}           Day price: {}€    
       | Location: {}       Month price: {}€     
       | ID:{}              Year price: {}€",center.name,center.day_price,center.location,center.month_price,id,center.year_price);
       id+=1;
    }
    print!("      ----------------------------------------\n");
}

// Function to read lines from a file
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


    // Now you have all fitness center information in `fitness_centers` vector
    // You can further process or display this information as needed
    let mut id=0;
    for center in &fitness_centers {
        println!("      ----------------------------------------
       | Name: {}           Day price: {}€    
       | Location: {}       Month price: {}€     
       | ID:{}              Year price: {}€",center.name,center.day_price,center.location,center.month_price,id,center.year_price);
       id+=1;
    }
    print!("      ----------------------------------------\n");
    println!("Enter the ID of the fitness center you want to rate:");
    let mut selected_id = String::new();
    io::stdin().read_line(&mut selected_id).expect("Failed to read input");
    let selected_id: usize = selected_id.trim().parse().expect("Invalid ID");

    if selected_id >= fitness_centers.len() {
        println!("Invalid ID!");
        return;
    }

    let selected_center = &mut fitness_centers[selected_id];

    // Collect ratings for different aspects
    println!("Enter ratings (0-5) for the following aspects:");
    println!("Cleanliness:");
    let mut clean_rating = String::new();
    io::stdin().read_line(&mut clean_rating).expect("Failed to read input");
    let clean_rating: i32 = clean_rating.trim().parse().expect("Invalid rating");
    selected_center.clean += clean_rating;

    println!("Personal:");
    let mut personal_rating = String::new();
    io::stdin().read_line(&mut personal_rating).expect("Failed to read input");
    let personal_rating: i32 = personal_rating.trim().parse().expect("Invalid rating");
    selected_center.personal += personal_rating;

    println!("Equipment quality:");
    let mut equip_rating = String::new();
    io::stdin().read_line(&mut equip_rating).expect("Failed to read input");
    let equip_rating: i32 = equip_rating.trim().parse().expect("Invalid rating");
    selected_center.equip += equip_rating;

    println!("Overall satisfaction:");
    let mut whole_rating = String::new();
    io::stdin().read_line(&mut whole_rating).expect("Failed to read input");
    let whole_rating: i32 = whole_rating.trim().parse().expect("Invalid rating");
    selected_center.whole += whole_rating;

    println!("Service quality:");
    let mut service_rating = String::new();
    io::stdin().read_line(&mut service_rating).expect("Failed to read input");
    let service_rating: i32 = service_rating.trim().parse().expect("Invalid rating");
    selected_center.service+= service_rating;

    // Increase the number of raters for the selected fitness center
    selected_center.raaters += 1;
    selected_center.score=((selected_center.clean*2+selected_center.personal*2+selected_center.equip*3+selected_center.whole*1+selected_center.service*1)/(selected_center.raaters*9))as f32;
    // Rewrite the text file with updated ratings
    if let Ok(mut file) = File::create("src/fittnes_info.txt") {
        for center in &fitness_centers {
            writeln!(
                file,
                "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
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