use std::io;
use std::process::Command;
mod  user;
mod display_top;
mod fitness;

/*TODO poriesit : v hesle(decrypt), testing treba poriadny error handleing na rate a add, inspect, mozno komentare, nejaky rank system by mohol byt. 
odstranenie zbitocnych lineov,maybee gui?? */

fn main() {
    //Command na vstup do appky
    let mut step = String::new();
    let current_user:String;
    let mut current_user_score:i32;

    loop {
        println!("1: Login\n2: Register\n3: Exit");
        step.clear();
        io::stdin().read_line(&mut step).expect("Failed to read command.");

        if step.trim().is_empty(){
            println!("Failed to read input.");
        }
        else if step.trim()=="3"{
            return;
        }
        else if step.trim()=="2"{
            current_user = user::register();
            break;
        }
        else if step.trim()=="1"{
            current_user = user::login();
            break;
        }
        else {
            println!("Invalid input!");
        }
    }

    current_user_score = user::get_score(&current_user.trim());

    //Command pre pohyb v appke
    let mut app_command = String::new();

    //Chcel by som tu dat moznost sa odhlasit namiesto exit ale neviem ako
    loop{
//---------------------------- vymaze terminal je to prehladnejsie dal mi to chat nwm jak to funguje ale je to viac jak gui
            
                    let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                    eprintln!("Failed to clear terminal");
}
//-------------------------------
            println!("What would you like to do?
        1: Check your profile       2: Rate
        3: Diplay best Raters       4: Display best fitness centres
        5: Add a fitness centre     6: Display fitness centers
        7: Exit");

        app_command.clear();
        io::stdin().read_line(&mut app_command).expect("Failed to read command.");
        
        if app_command.trim().is_empty(){
            println!("Failed to read input.");
        }
        else if app_command.trim()=="1"{
            //---------------------------- 
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
            //-------------------------------
            println!("Name: {}Score: {}",current_user, current_user_score);
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
            //---------------------------- 
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
            //-------------------------------
            current_user_score +=1;
            fitness::rate_fittnes();
               }
        else if app_command.trim()=="3"{
            //---------------------------- 
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            //-------------------------------
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
            //---------------------------- 
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            //-------------------------------
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
            //---------------------------- 
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
            //-------------------------------
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
        else if app_command.trim()=="6"{
            //---------------------------- 
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
            //-------------------------------
            fitness::display_fitness();
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
                fitness::display_fitness();
            }
            
        }
        else if app_command.trim()=="7"{
            return;
        }
        else {
            println!("Invalid input!");
        }
    }
    
}
