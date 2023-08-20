#! [allow(unused)]
use std::io;

fn main() {
    let x=4;
    if x>10{
    println!("x is greater than 10");
    }
    else if x>=5 {
    println!("x is greater than 5 or equal to 5");
    }
    else{
        println!("x is lesser than 5");
    }

    loob();

    cool();

    pool();

    tool();

    ray();

    wool();
    
    
}

//if else

fn loob(){
    let y=false;
    let num= if y {9} else {7};
    println!("num is: {}",num);
}
//loop

fn cool() {
    let mut counter =0;
    let result = loop {
        counter+=1;
        
        if counter==10 {
            break counter * 2;
        }
    };

    println!("result is:{}",result);
}

// outer and inner loops

fn pool(){
    let mut count = 0;
    'count_up: loop {
        println!("count is : {}",count);
        let mut rem_count=10;
        loop {
        println!("remaining = {}",rem_count);    
        if rem_count==8{
            break;
        }
        if count == 3{
            break 'count_up;
        }
        rem_count-=1;
    }
     count+=1;   

    }
    println!("count is : {}",count);

}
//while loop

fn tool(){
    let mut p = 0;
    while p<10{
        p+=1;
        println!("counting till 10 {}",p);
    }
    println!("we have counted till ten");
}

//array with while

fn ray(){
    let v = [10,20,30,40,50];
    let mut index=0;
    while index<5{
        println!("value at index {} is {}",index,v[index]);
        index+=1;
        
    }
}

// for loop
fn wool(){
    let k = [60,70,80,90,100];
    for ken in k {
        println!("elements of k are: {}",ken);
    }

    {
        for nun in (1..9).rev(){
            println!("nuns are: {}",nun);
        }
    }

    println!("DONE WITH FLOW CONTROL");
}