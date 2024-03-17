use std::io;
struct QQQ{
    Question:String,
    answer1:bool,
    answer2:bool,
    answer3:bool,
    answer4:bool,

}

fn question_input(){
    let mut question = String::new();

    // Read a line of input from the user
    io::stdin().read_line(&mut question)
        .expect("Failed to read line");

    // Trim any whitespace from the input
    question = question.trim().to_string();
    
    let otazka= QQQ{
        Question:String::from(question),
        answer1:false,
        answer2:false,
        answer3:false,
        answer4:true,
    };
}
fn quizzing(){
    println!("user");
}



fn main() {

    println!("select mode:\n q:quiz mode \n e:entering questions");
    let mut input = String::new();

    // Read a line of input from the user
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Trim any whitespace from the input
    input = input.trim().to_string();

    // Ensure that the input is exactly one character
    if input.len() != 1 {
        println!("Error: Please enter exactly one character.");
        return;
    }

    if input=="q"{
        question_input();
    }
    else if input=="e"{
        quizzing();
    }
    else{
        println!("wrong input");
    }
   
}
