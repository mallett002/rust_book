use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Error reading file");

    println!("With text: {contents}");
    // TODO: left off https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Returns result of success (Config) and Error (string literal)
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // ensure we have enough args
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // parse the arguments
        let query = args[1].clone();
        let file_path = args[2].clone();

        // create the config
        Ok(Config { query, file_path })
    }
}
