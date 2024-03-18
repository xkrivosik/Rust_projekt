use std::io;

fn is_prime(cislo: i32)->i32{
    let mut delitelpoc=0;
    let mut delitel=1;
    while delitel<=cislo{
        if cislo % delitel == 0 {
            delitelpoc+=1;
        }
        delitel+=1;
    }
    if delitelpoc>2{
        return 0;
    }
    return 1;
}

fn main() {
    let mut input =String::new();
    println!("Enter a number to check if its prime:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let cislo: i32 = input.trim().parse().expect("Failed to read number");

    let vysledok = is_prime(cislo);

    if vysledok == 1{
        println!("Number: {} is prime",cislo);
    }
    else{
        println!("Number: {} is not prime",cislo);
    }
}
