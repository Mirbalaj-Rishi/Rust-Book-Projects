use std::io;
fn main() {
    const ARRAY_SIZE:usize = 15;
    let mut array: [usize; ARRAY_SIZE] = [0; ARRAY_SIZE];
    println!("Please type in the nth Fibonacci Number you would like to see from 0 to 500");
    let mut user_amount = String::new();
    io::stdin().read_line(&mut user_amount).expect("failed to read line");
    let user_amount:usize = match user_amount.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };
    
    array[0] = 0;
    array[1] = 1;
    let mut i_prev:usize = 1;
    let mut i_fut:usize = 1;

    for i in 1..user_amount{
        println!("{i}");
        i_prev = i - 1;
        i_fut = i + 1;
        array[i_fut] = array[i_prev] + array[i];
        

    }
    let awnser = array[user_amount-1];
    

    println!("The {user_amount}th fibonacci number is {awnser}");
    
    for i in array{
        println!("{i}");
    }


    

}
