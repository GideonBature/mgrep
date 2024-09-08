use std::{env, process};

use mgrep::{Config, run};

fn main() {
    // Read the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    // let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}



// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
    
//     Config { query, file_path }
// }
