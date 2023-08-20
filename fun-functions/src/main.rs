use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");
    loop{
   
    println!("enter the temperature in Farhenheit");

    let mut farh =String::new();
    io::stdin().read_line(&mut farh).expect("You have not entered a number");
     let farh:f32 = farh.trim().parse().expect("enter the proper number");

    let celsius = (farh-32.0)*5.0/9.0;
    println!("Temperature in celsius is:{}",celsius);
    }
    
}
