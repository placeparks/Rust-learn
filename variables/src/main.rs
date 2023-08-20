fn main() {
    let mut x = 5;

    println!("the value of x is : {}",x);

    x = 6;

    println!("the value of x2 is : {}",x);

    let x=x+5;

    println!("the value of x1 is : {}",x);

    const SEC_IN_HOUR : u32 = 60*60;

    println!("Total seconds in 1 hour is: {} ",SEC_IN_HOUR);

    {
        let y:u32= x*9;
        println!("x multiplied by 9 is: {} ",y);
        let total_sec:u32 = y*SEC_IN_HOUR;
        println!("total second in y hours is: with y AS : {} {} ",total_sec,y);
    }

    
}
