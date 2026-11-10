use std::thread;

fn main() {
    println!("");
    if_let_else();

    println!("");
    while_let();

    println!("");
    for_loops();

    println!("");
    function_params();

    println!("");
    refutability();

    println!("");
    matching_literals();

    println!("");
    named_parameters();
}

fn if_let_else() {
    println!("if_let_else:");
    // unlike match, if let else branches don't need to relate to one another - more flexible
    // downsides of if let: arms aren't checked for exhaustiveness like match does

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using favorite color {color} as background.");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color.");
        } else {
            println!("Using orange as background color.");
        }
    } else {
        println!("Using blue as background color.");
    }
}

fn while_let() {
    println!("while_let:");
    // runs while loop for as long as pattern matches in condition

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        for n in [1, 2, 3] {
            tx.send(n).unwrap();
        }
    });

    while let Ok(n) = rx.recv() {
        println!("received {n}");
    }
}

fn for_loops() {
    println!("for_loops:");
    // for x in y (x is the pattern)

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

// parameters are patterns (can "destructure" them to match on)
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn function_params() {
    println!("function_params:");

    let point = (5, 3);

    print_coordinates(&point);
}

fn refutability() {
    println!("refutability:");

    // handle refutable patterns
    let some_option_value: Option<i32> = None;

    // have to handle else case for Some(val), bc refutable pattern (could be a None)
    let Some(val): Option<i32> = some_option_value else {
        return;
    };


    // Will give warning, rust knows this is an irrefutable pattern (x), so the else isn't needed
    let x = 5 else {
        return;
    };
}

fn matching_literals() {
    println!("matching_literals:");

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything!"),
    }
}

fn named_parameters() {
    println!("named_parameters:");

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // matches here (matches any value inside a Some)
        _ => println!("Default case, x = {x:?}"),
    };

    println!("At the end: x = {x:?}, y = {y}"); // named param out of scope now, so y = 10
}
