use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {

    //welcome prompt

    println!("Welcome to the Guess Game!");

    //generating random number from 1 to 100 in this line 

    let guess_number = rand::thread_rng().gen_range(1..=100);
    // println!("secret number is:{}",guess_number);

    //loop logic of multiple inputs for game continuity till user successfully guesses the number

    loop {

    println!("Enter the guess Number");

    // Defining an empty mutable string in Rust

    let mut guess = String::new();

    //Taking the input from user and storing it in guess variable refrence using input output library in Rust

    io::stdin().read_line(&mut guess).expect("Could not read a number");

    //Converting String type into Integer here and handling error with match statement in case
    //user enter something other than numbers

    let guess:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=> continue,
    };

    println!("Guessed number is:{guess}");

    // using match function of Rust to compare random number and guessed number
    // specifying every condition in match body and breaking the loop if both number are equal


    match guess.cmp(&guess_number){
         Ordering::Less => println!("ALAS!The number you guessed is too Low.Better Luck Next Time!"),
         Ordering::Greater => println!("ALAS!The number you guessed is too High.Better Luck Next Time!"),
         Ordering::Equal => {
            println!("HURRAH! You have Guessed the right Number");
            break;
         }
    }
 }

}
