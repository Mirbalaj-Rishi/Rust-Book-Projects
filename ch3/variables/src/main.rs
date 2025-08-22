fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //constants are always immutable
    //can be the result of an expression
    //can't be the result of a runtime calculation
    //other parts of program can use this

    let mut x = 5; //mut makes it a mutable variable without mut it wont compile since we are changing x
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    //shadowing 
    let y = 5;
    let y = y + 1;
    {//inner scope
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }
    println!("The value of x is: {y}");
    //"By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed."

    //shadowing can change variable types, this would not work with mut
    let spaces = "   ";
    let spaces = spaces.len();
}