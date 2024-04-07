//use std::fs::OpenOptions;
//use std::io::{self, Write};
use std::io;
mod  user;
mod display_top;
mod fitness;
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
            println!("Name: {}\nScore: {}",current_user, current_user_score);
            let mut back = String::new();

            loop {
                println!("Press 'e' to go back");
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
        else if app_command.trim()=="2"{
            println!("Rating tu (zatial iba da +1 do skore pre testovanie)");
            current_user_score +=1;
        }
        else if app_command.trim()=="3"{
            display_top::top_rates();
            break;
        }
        else if app_command.trim()=="4"{
            println!("Best centra tu");
            display_top::top_fitness();
            break;
        }
        else if app_command.trim()=="5"{
            println!("Adding tu");
            fitness::add_fit();
            break;
        }
        else if app_command.trim()=="6"{
            //search tu 
            return;
        }
        else if app_command.trim()=="7"{
            return;
        }
        else {
            println!("Invalid input!");
        }
    }
    
}
