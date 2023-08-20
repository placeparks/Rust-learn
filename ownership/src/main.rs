
fn main(){
    let mut ken = String::from("helloww");
    let length= length_ken(&ken);
  
    println!("the length of {} is {}",ken,length);
}

fn length_ken(s: &String)->usize{
    s.len()
}