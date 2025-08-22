use std::io; //import standard IO for input/output

fn main() {
    let mut user_choice = String::new();
    println!("Type f for Fahrenheit and c for Celsius");
    io::stdin()
            .read_line(&mut user_choice) //store in guess
            //&mut is a referece that allows mutiple things to access the mutable string guess
            .expect("Failed to read line");
    let mut user_value = String::new();
    let convert:f64;
    println!("please type in the amount");
    io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line"); 
    let user_value = user_value.trim().parse::<f64>().unwrap();
    if user_choice.trim() == "f"{
        println!("Fahrenheit selected");
        convert = (user_value - 32.0)/1.8; //must be 32.0 since it is a float
        println!("{user_value}°F = {convert}°C");
    }else if user_choice.trim() == "c" {
        println!("Celsius selected please");
        convert = (user_value *1.8) + 32.0; //must be 32.0 since it is a float
        println!("{user_value}°C = {convert}°F");
    }
    

}

