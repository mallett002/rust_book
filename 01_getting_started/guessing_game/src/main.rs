use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    // I'm adding this
    const MAX_GUESSES: u32 = 10;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut total_guesses: u32 = 1;

    loop {
        println!("Guess # {total_guesses} of {MAX_GUESSES}.");

        // init user's guess var
        let mut guess = String::new();

        // read in user's guess (string)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse the guess into a number
        // ignore non numbers
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // I added this (guess the # within 10 tries):
        if total_guesses == MAX_GUESSES {
            println!("You Lose!");
            println!("The number was {secret_number}");
            break;
        }

        total_guesses += 1;
    }
}
