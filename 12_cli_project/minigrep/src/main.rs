use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Error reading file");

    println!("With text: {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // ensure we have enough args
        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        // parse the arguments
        let query = args[1].clone();
        let file_path = args[2].clone();

        // create the config
        Config { query, file_path }
    }
}
