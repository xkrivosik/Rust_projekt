use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process::Command;
use std::path::Path;

use chrono::{DateTime, Local};

use  crate::user;

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
    
    let selected_id= inspect() as i32;
    if selected_id==-1 {
        return;
    }

    let selected_center = &mut fitness_centers[selected_id as usize];

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

pub fn inspect() -> i32 {
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

    let mut display_index = 0;

    loop {
        let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

        if !output.success() {
            eprintln!("Failed to clear terminal");
        }
        display_fitnessi(&fitness_centers, display_index);
        match count_lines_in_file("src/fittnes_info.txt") {
            Ok(count) => println!("Showing 5 out of {} centers", count),
            Err(err) => eprintln!("Error: {}", err),
        }
        println!("Enter the ID of the fitness center you want to inspect:\n'n' -> next page\n'p' -> previous page\n'e' -> exit");
        let mut input_id = String::new();
        io::stdin()
            .read_line(&mut input_id)
            .expect("Failed to read input");

        let input_id = input_id.trim();

        if input_id == "e" {
            return -1 as i32;
            
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
                if display_index + 10 < fitness_centers.len() {
                    display_index += 10;
                } else {
                    let remaining = fitness_centers.len() - display_index - 1;
                    println!("You're already at the last page with {} records.", remaining);
                }
            }
         
            _ => {
                if let Ok(parsed_index) = input_id.parse::<usize>() {
                    if parsed_index < fitness_centers.len() {
                        display_index = parsed_index;
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
    return  display_index as i32
}
fn display_fitnessi(fitness_centers: &Vec<FitnessCenter>, start_index: usize) {
    const PAGE_SIZE: usize = 10;

    println!("Fitness Centers:");
    for (i, fitness_center) in fitness_centers.iter().enumerate().skip(start_index).take(PAGE_SIZE) {
        println!("ID: {} - Name: {}", i, fitness_center.name);
    }
}
struct ComentLog {
    id: usize,
    indexx: String,
    comment: String,
    like: usize,
    dislike: usize,
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
    let  index=inspect();
    
    if index==-1{
        return;
    }
    let mut back = String::new();
    loop {
        let output = Command::new("cmd")
                 .args(&["/C", "cls"])
                 .status()
                 .expect("Failed to clear terminal");

                 if !output.success() {
                      eprintln!("Failed to clear terminal");
                             }
                            
        if (index as usize) < fitness_centers.len() {
                println!("-----------------------------------------------------------------------------------");
                println!("| ID: {}  Name: {}", index, fitness_centers[index as usize].name);
                println!("| Location: {}",fitness_centers[index as usize].location);
                println!("| One-time entry: {}€",fitness_centers[index as usize].day_price);
                println!("| Monthly subscription: {}€",fitness_centers[index as usize].month_price);
                println!("| Yearly subscription: {}€",fitness_centers[index as usize].year_price);
                println!("-----------------------------------------------------------------------------------");
                println!("| Rating: {:.1}/5 ({})\tServices: {:.1}/5 ",fitness_centers[index as usize].score,fitness_centers[index as usize].raaters,(fitness_centers[index as usize].service as f32)/(fitness_centers[index as usize].raaters as f32));
                println!("| Cleanleness: {:.1}/5\tPersonal: {:.1}/5 ",(fitness_centers[index as usize].clean as f32)/(fitness_centers[index as usize].raaters as f32),(fitness_centers[index as usize].personal as f32)/(fitness_centers[index as usize].raaters as f32));
                println!("| Equipment: {:.1}/5\tOver all experience: {:.1}/5 ",(fitness_centers[index as usize].equip as f32)/(fitness_centers[index as usize].raaters as f32),(fitness_centers[index as usize].whole as f32)/(fitness_centers[index as usize].raaters as f32));
                println!("-----------------------------------------------------------------------------------");
                println!("Comment section:");
        } else {
                println!("Index out of bounds!");
                }
                display_coment(index as usize);
                
        println!("\nPress 'e' to go back, 'c' to comment fitness center or 'r' to rate comment");
        back.clear();
        io::stdin().read_line(&mut back).expect("Failed to read command.");

        
        if back.trim()=="e"{
        break;
        }
        else if back.trim()=="c"{
            comment(username,index as usize);
        }
        else if back.trim() == "r" {
            rate_comment(index as usize);
        }
        
    }
     
}
fn rate_comment(index: usize) {
    let mut comments: Vec<ComentLog> = Vec::new();
    let mut good_comments: Vec<ComentLog> = Vec::new();

    // Read lines from the file
    if let Ok(file) = File::open("src/comment.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split('|').collect();

                if parts.len() == 5 {
                    let id = parts[0].parse::<usize>().unwrap_or(0);
                    let indexx = parts[1].to_string();
                    let comment = parts[2].to_string();
                    let like = parts[3].parse::<usize>().unwrap_or(0);
                    let dislike = parts[4].parse::<usize>().unwrap_or(0);

                    let comment_log = ComentLog {
                        id,
                        indexx: indexx.clone(),
                        comment: comment.clone(),
                        like,
                        dislike,
                    };

                    comments.push(comment_log);

                    // Check if the ID matches the given index
                    if id == index {
                        good_comments.push(ComentLog {
                            id,
                            indexx,
                            comment,
                            like,
                            dislike,
                        });
                    }
                }
            }
        }
    } else {
        println!("Failed to open file.");
        return;
    }
   
    

    println!("Enter the index of the comment to rate:");
    let mut poradie = String::new();
    io::stdin().read_line(&mut poradie).expect("Failed to read input");

    let hladany_index: usize = match poradie.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Press 'l' to like or 'd' to dislike:");
    let mut reaction = String::new();
    io::stdin().read_line(&mut reaction).expect("Failed to read input");
    //upravit na mapovanie funkcii mame pole good_comments ktore obsahuje len comenty ku danemu fitku 
    if reaction.trim() == "l" {
        for  com in &mut comments{
            if good_comments[hladany_index].indexx==com.indexx{
                let splited:Vec<&str>=good_comments[hladany_index].comment.split(':').collect();
                let name= splited[0].trim().to_string();
                user::score_update(name);
                com.like += 1;
            }
        }
        /*if comments[hladany_index].id == index  {
            comments[hladany_index].like += 1;
            let qd = comments[hladany_index].comment.clone();
            let pole: Vec<&str> = qd.split(':').collect();
            user::score_update(pole[0].trim().to_string().clone());
        }*/
    } else if reaction.trim() == "d" {
        for  com in &mut comments{
            if good_comments[hladany_index].indexx==com.indexx{
                com.dislike += 1;
            }
        }
    } else {
        println!("Invalid reaction.");
        return;
    }

    // Open the file in write mode and handle errors
    let file = match File::create("src/comment.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to create file: {}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    // Write the updated comments to the file
    for comment in &comments {
        if let Err(e) = writeln!(
            writer,
            "{}|{}|{}|{}|{}",
            comment.id,comment.indexx, comment.comment, comment.like, comment.dislike
        ) {
            println!("Failed to write comment: {}", e);
            return;
        }
    }

    // Flush the buffer to ensure all data is written
    if let Err(e) = writer.flush() {
        println!("Failed to flush buffer: {}", e);
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
let time_index = current_time_and_date();
writeln!(file, "{}|{}|{}:{}|0|0", index,time_index, username.trim(), coment.trim())
    .expect("Failed to write to file");
println!("comment added");

}
fn current_time_and_date() -> DateTime<Local> {
    Local::now()
}


fn display_coment(index: usize) {
    let mut comments: Vec<ComentLog> = Vec::new();

    if let Ok(lines) = read_lines("src/comment.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split('|').collect();

                if parts.len() == 5{
                    let id = parts[0].parse::<usize>().unwrap_or(0);
                    let indexx = parts[1].to_string();
                    let comment = parts[2].to_string();
                    let like = parts[3].parse::<usize>().unwrap_or(0);
                    let dislike = parts[4].parse::<usize>().unwrap_or(0);

                    comments.push(ComentLog {
                        id,
                        indexx,
                        comment,
                        like,
                        dislike,
                    });
                }
            }
        }
    }
    let mut count =0;
    for comment in &comments {
        if comment.id == index {
            println!("{}. {}
like:{} | dislike:{}
", count,comment.comment,comment.like,comment.dislike);
count+=1;  
        }
        
    }

}

fn count_lines_in_file(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut line_count = 0;

    for _ in reader.lines() {
        line_count += 1;
    }

    Ok(line_count)
}