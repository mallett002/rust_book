use std::env;
use std::error::Error;
use std::fs;
use std::process;

// main is only in charge of parsing the arguments and sending them to the run fn
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    run(config).unwrap_or_else(|err| {
        println!("run error: {err}");
    });

    // TODO: working on https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#handling-errors-returned-from-run-in-main
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: {contents}");

    Ok(())
}
