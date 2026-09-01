use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

// example usage:
// Basic: `cargo run -- to poem.txt`
// Ignore case with env var: `IGNORE_CASE=1 cargo run -- to poem.txt` ->
// Turn off ignore case with flag (overrides env var): `cargo run -- to poem.txt --ignore_case=false`

// main is only in charge of parsing the arguments and sending them to the run fn
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
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

        // set ignore_case from env var
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        // check for --ignore_case flag (overrides env var)
        if args.len() > 3 {
            ignore_case = parse_flags(args);
        }

        // create the config
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// I added this
fn parse_flags(args: &[String]) -> bool {
    let flag = args[3].clone();

    // ensure it's --ignore_case=true|false
    let mut key_val_iterator = flag.split("=");

    let key = key_val_iterator.next().unwrap();

    if key != "--ignore_case" {
        println!("invalid flag: {key}");
        process::exit(1);
    }

    // get the bool and set ignore_case to it
    match key_val_iterator.next().unwrap() {
        "true" => true,
        "false" => false,
        _ => false,
    }
}
