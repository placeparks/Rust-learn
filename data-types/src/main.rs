#! [allow(unused)]
use std::io;



fn main() {
 //integers 
    
    println!("Hello, Mirac!");
    let x = 3.6;
    let y = 6.3;
    let z = y*x;
    println!("the product is: {} ",z);
//bool
    let t = true;
    let f = false;
    println!("{} {}",t,f);
//character
    let c = 'z';
    println!("{}",c);
//string
    let s = "Nice Day!";
    println!("{}", s);
//tuple
    let tup = (200,"love",2.5,'a',"Hate");
    let (_a,_b,_c,_d,_e) = tup;
    let b = tup.4;
    println!(" b and _b are  {} {} ",b,_b );


 //arrays
    let arr = [1,2,3,4,5,6];
    let arrr = [9;6];
    println!("{} {} ", arrr[0], arr[4]);

    println!("enter the Index");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read line");
    let index:usize = index.trim().parse().expect("index entered was not a number");
    let element = arr[index];
    println!("element of arr at index{} is: {}",index,element);








}
