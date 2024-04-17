use std::io;
use std::process::Command;
mod  user;
mod display_top;
mod fitness;

/*TODO V inspecte neukazuje pocet ratingov z nejakeho dovodu
maybee gui?? */

fn main() {
    //Command na vstup do appky
    let mut step = String::new();
    let current_user:String;
    let mut current_user_score:i32;
    let mut current_user_rank:String;

    loop {
        println!("1: Login\n2: Register\n3: Exit\n\nEnter command: ");
        step.clear();
        io::stdin().read_line(&mut step).expect("Failed to read command.");

        if step.trim().is_empty(){
            println!("Failed to read input.");
        }
        else if step.trim()=="3"{
            return;
        }
        else if step.trim()=="2"{
            let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
            current_user = user::register();
            break;
        }
        else if step.trim()=="1"{
            let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
            current_user = user::login();
            break;
        }
        else {
            println!("Invalid input!");
        }
    }

    //Command pre pohyb v appke
    let mut app_command = String::new();

    //Chcel by som tu dat moznost sa odhlasit namiesto exit ale neviem ako
    loop{
        current_user_score = user::get_score(&current_user.trim());
        current_user_rank = user::get_rank(current_user_score);            
        let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

            if !output.success() {
                eprintln!("Failed to clear terminal");
            }

        println!("What would you like to do?
        1: Check your profile       2: Rate
        3: Inspect fitness          4: Display best fitness centres
        5: Add a fitness centre     6: Diplay best Raters
        7: Exit");
        println!("Enter command:");
        
        app_command.clear();
        io::stdin().read_line(&mut app_command).expect("Failed to read command.");
        
        if app_command.trim().is_empty(){
            println!("Failed to read input.");
        }
        else if app_command.trim()=="1"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            println!("Name: {}Score: {}\nRank: {}",current_user, current_user_score, current_user_rank);
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }

                let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
                println!("Name: {}Score: {}",current_user, current_user_score);
            }
        }
        else if app_command.trim()=="2"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            
            fitness::rate_fittnes();
            user::score_update(current_user.trim().to_string().clone());
               }
        else if app_command.trim()=="6"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            display_top::top_rates();
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }

                let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
                display_top::top_rates();
            }
        }
        else if app_command.trim()=="4"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            display_top::top_fitness();
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }

                let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
                display_top::top_fitness();
            }
            
        }
        else if app_command.trim()=="5"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            fitness::add_fit();
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }
            }
            
        }
        else if app_command.trim()=="3"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            fitness::inspection(&current_user);
            
        }
        else if app_command.trim()=="7"{
            return;
        }
        else {
            println!(" ");
        }
    }
    
}
