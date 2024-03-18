use std::io;
use num_traits::Float;

fn round_to<T: Float>(number: T, decimal_places: usize) -> T {
    let factor = T::from(10).unwrap().powi(decimal_places as i32);
    (number * factor).round() / factor
}


fn main() {
    let mut input = String::new();
    let mut input_choice = String::new();

    println!("Enter a temperature and type for degrees\n-> C - Celsius\n-> F - Fahrenheit\n-> K - Kelvin\nexample: 37 C");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut casti = input.trim().split_whitespace();

    let value: f64 = match casti.next(){
        Some(num_str) => match num_str.parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid format for the value.");
                return ;
            }
        },
        None => {
            println!("Invalid input format.");
            return;
        }
    };

    let typ: char = match casti.next(){
        Some(sym) => sym.chars().next().unwrap(),
        None => {
            println!("INvalid input format.");
            return;
        }
    };

    println!("To which type would you like to convert to?\n-> C - Celsius\n-> F - Fahrenheit\n-> K - Kelvin");
    io::stdin().read_line(&mut input_choice).expect("Failed to read line.");

    let typ_choice = input_choice.chars().next().unwrap();

    if typ_choice == 'K' || typ_choice == 'k'{
        if typ == 'K' || typ == 'k'{
            println!("{} {} in Kelvin is {} {}",value,typ,value,typ_choice);
        }
        if typ == 'C' || typ == 'c'{
            println!("{} {} in Kelvin is {} {}",value,typ,round_to(value+273.15,2),typ_choice);
        }  
        if typ == 'F' || typ == 'f'{
            println!("{} {} in Kelvin is {} {}",value,typ,round_to((value-32.0)*(5.0/9.0)+273.15,2),typ_choice);
        }          
    }
    if typ_choice == 'C' || typ_choice == 'c'{
        if typ == 'K' || typ == 'k'{
            println!("{} {} in Celsius is {} {}",value,typ,round_to(value-273.15,2),typ_choice);
        }
        if typ == 'C' || typ == 'c'{
            println!("{} {} in Celsius is {} {}",value,typ,value,typ_choice);
        }  
        if typ == 'F' || typ == 'f'{
            println!("{} {} in Celsius is {} {}",value,typ,round_to((value-32.0)*(5.0/9.0),2),typ_choice);
        }          
    }
    if typ_choice == 'F' || typ_choice == 'f'{
        if typ == 'K' || typ == 'k'{
            println!("{} {} in Fahrenheit is {} {}",value,typ,round_to((value-273.15)*(9.0/5.0)+32.0,2),typ_choice);
        }
        if typ == 'C' || typ == 'c'{
            println!("{} {} in Fahrenheit is {} {}",value,typ,round_to((value)*(9.0/5.0)+32.0,2),typ_choice);
        }  
        if typ == 'F' || typ == 'f'{
            println!("{} {} in Fahrenheit is {} {}",value,typ,value,typ_choice);
        }          
    }
}
