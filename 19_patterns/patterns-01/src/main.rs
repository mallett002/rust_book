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
