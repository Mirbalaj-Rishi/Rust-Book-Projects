use std::io; //import standard IO for input/output
use rand::Rng;
use std::cmp::Ordering; //imports .cmp

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);//get a random number from 1 to 100

    loop{
        println!("Please input your guess.");
        let mut guess = String::new(); //create a variable to store user input. String::new() creates an empty string

        io::stdin()
            .read_line(&mut guess) //store in guess
            //&mut is a referece that allows mutiple things to access the mutable string guess
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //(_) means take all values that cuase an error and go to the next item in the loop if there are any errors
        };
        
        //we are shadowing guess to overwrite it with a u32 instead of a string
        //u32 is an unsigned 32 bit number
        //.trim gets rid of white spaces at the start and end
        //.parse converts a string to a new type

        
        println!("you guessed: {guess}");

            match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
            println!("You win!");
            break;
            }
        }//rust infers that secret_number should be u32 just like guess
        }
}

