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
                break 'counting_up // break the outer loop 
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
