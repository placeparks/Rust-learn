fn main() {

    let mut s =String::from("hello");
     s.push_str(",world!");


    println!("{}",s);

    let d=String::from("nicee");
    let d1=d.clone();
    println!("{}={}",d,d1);

    takes_ownership(s);
}


fn takes_ownership(any_string: String){
    print!("{}",any_string);
}
