// import the env module
use std::env;

fn main() {
    // Read the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Print the arguments
    dbg!(&args);

    // Save the arguments in variables
    let arg1 = &args[1];
    let arg2 = &args[2];

    println!("arg1: {}, arg2: {}", arg1, arg2);


}
