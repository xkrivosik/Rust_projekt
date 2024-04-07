use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

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
    writeln!(file, "{}:{}:{}:{}:{}:0:0:0:0", name.trim(), location.trim(), den_vstup, mes_vstup, rok_vstup)
        .expect("Failed to write to file");
    println!("fitness center: {} is added",name.trim());

    

}
pub fn display_fitness(){
    println!("display here");
}