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
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

// new way with iterators
impl Config {
    // Returns result of success (Config) and Error (string literal)
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip first item (name of program, we don't care ab it)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let ignore_case = match args.next() {
            Some(arg) => parse_ignore_case(arg),
            None => env::var("IGNORE_CASE").is_ok(),
        };

        // create the config
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// build method old way (without iterators)
// impl Config {
//     // Returns result of success (Config) and Error (string literal)
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         // ensure we have enough args
//         if args.len() < 3 {
//             return Err("Not enough arguments");
//         }
//
//         // parse the arguments
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//
//         // set ignore_case from env var
//         let mut ignore_case = env::var("IGNORE_CASE").is_ok();
//
//         // check for --ignore_case flag (overrides env var)
//         if args.len() > 3 {
//             ignore_case = parse_flags(args);
//         }
//
//         // create the config
//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//         })
//     }
// }

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        let case_insensitive_results = search_case_insensitive(&config.query, &contents);

        case_insensitive_results.collect()
    } else {
        let case_sensitive_results = search(&config.query, &contents);

        case_sensitive_results.collect()
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn parse_ignore_case(flag: String) -> bool {
    let mut key_val_iterator = flag.split("=");
    let key = key_val_iterator.next().unwrap();

    if key != "--ignore_case" {
        eprintln!("invalid flag: {key}");
        process::exit(1);
    }

    // get the bool and set ignore_case to it
    match key_val_iterator.next().unwrap() {
        "true" => true,
        "false" => false,
        _ => false,
    }
}
