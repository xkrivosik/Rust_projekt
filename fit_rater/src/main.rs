//use std::fs::OpenOptions;
//use std::io::{self, Write};
use std::io;
mod  createacc;
fn main() {
    let mut step = String::new();

    loop {
    

    println!("1: create acount\n2: login\n3: exit");
    step.clear();
    io::stdin().read_line(&mut step).expect("Failed to read password");

   if step.trim().is_empty(){
        println!("invalid input!!!");
    }
    else if step.trim()=="3"{
        break;
    }
    else if step.trim()=="1"{
        createacc::new_acc();
    }
    else if step.trim()=="2"{
        println!("login good");
    }
    else {
        break;
    }
}
    
}
