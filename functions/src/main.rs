fn main() {
    println!("Hello, world!");

    another_function(45,9);
    
    println!("the value is {}",add(9,8));

    
}

fn another_function(x:i32,y:i32){
    println!("sum of x and y is {}",x+y);
    println!("product of x and y is {}",x*y);
    println!("division of x and y is {}",x/y);
    println!("subtraction of x and y is {}",x-y);
    println!("remainder of x % y is {}",x%y);

    
}

fn add(x:i32,y:i32)->i32{
   x+y
   
}