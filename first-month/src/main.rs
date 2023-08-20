#! [allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;




fn main() {
    println!("what is your name?");
    let mut ken : String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut ken)
    .expect("Didn't receive your input");
    println!("Hello {} {} ", ken.trim_end(), greeting );
}
