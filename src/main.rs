// import the env module
use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn main() {
    // Read the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
    
//     Config { query, file_path }
// }
