use std::collections::HashMap;

fn main() {
    // loop that doesn't end:
    // loop {
    //     println!("Hello, world!");
    // }

    returning_values_from_loops();
    loop_labels();
    while_loops();
    looping_through_collections();
    alternative_to_while_is_range();
    temps_program();
    gen_nth_fib_program();
    lyrics_12_days_christmas_program();
}

fn returning_values_from_loops() {
    println!("\nreturning_values_from_loops:");

    let mut counter = 0;

    // save result of loop to value
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // "break" exits loop and saves to var
    // "return" could be used too, but would exit the whole function
}

fn loop_labels() {
    println!("\nloop_labels:");

    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");

        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                println!("breaking inner");
                break;
            }
            if count == 2 {
                println!("breaking outer!");
                break 'counting_up; // break the outer loop 
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_loops() {
    println!("\nwhile_loops:");

    let mut number = 3;

    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn looping_through_collections() {
    println!("\nlooping_through_collections:");

    let a = [10, 20, 30, 40, 50];

    // 1. Using while loop: (bit slower due to checking if out of bounds)
    let mut index = 0;

    while index < 5 {
        println!("the values is: {}", a[index]);

        index += 1;
    }

    // 2. Using "for in" - cleaner and more efficient
    for item in a {
        println!("the values is: {item}");
    }
}

fn alternative_to_while_is_range() {
    println!("\nalternative_to_while_is_range");

    for number in 1..4 {
        println!("number: {number}"); // 1 2 3
    }

    // can reverse it with "rev"
    for number in (1..4).rev() {
        println!("number: {number}"); // 3 2 1
    }
}

fn temps_program() {
    let fahr_temp: f32 = 100.0;
    let celc_temp: f32 = convert_to_celsius(fahr_temp);

    println!("{fahr_temp} fahrenheit converted to celsius is {celc_temp}");
    println!(
        "converted back to fahrenheit is {}",
        convert_to_fahrenheit(celc_temp)
    );
}

fn convert_to_celsius(f: f32) -> f32 {
    ((f - 32.0) * 5.0 / 9.0).round()
}

fn convert_to_fahrenheit(c: f32) -> f32 {
    ((c * 9.0 / 5.0) + 32.0).round()
}

fn gen_nth_fib_program() {
    let nth_fib_number = 4;

    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144
    println!("fib({nth_fib_number}): {}", fib(nth_fib_number));
}

fn fib(seq: u8) -> u8 {
    if seq == 1 {
        return 0;
    }

    // resume on 2nd iteration:
    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..seq {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn lyrics_12_days_christmas_program() {
    let song_template = HashMap::from([
        ("first", "A partridge in a pear tree"),
        ("second", "Two turtle doves and"),
        ("third", "Three french hens,"),
        ("fourth", "Four calling birds,"),
        ("fifth", "Five golden rings,"),
        ("sixth", "Six geese a-laying,"),
        ("seventh", "Seven swans a-swimming,"),
        ("eighth", "Eight maids a-milking,"),
        ("ninth", "Nine ladies dancing,"),
        ("tenth", "Ten lords a-leaping,"),
        ("eleventh", "Eleven pipers piping,"),
        ("twelfth", "Twelve drummers drumming,"),
    ]);

    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut gifts: Vec<&str> = Vec::new();

    for day in days {
        let gift = match song_template.get(&day) {
            // or just song_template[day];
            Some(&res) => res,
            None => "",
        };

        gifts.push(gift);

        println!("On the {day} day of Christmas my true love gave to me:");

        for g in gifts.iter().rev() {
            println!("{g}");
        }

        println!();
    }
}
