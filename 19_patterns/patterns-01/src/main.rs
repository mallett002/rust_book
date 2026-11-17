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

    println!("");
    multiple_pattern_matching();

    println!("");
    range_matching();

    println!("");
    destructuring();

    println!("");
    enum_matching();

    println!("");
    nested_enum_matching();

    println!("");
    struct_tuple_matching();

    println!("");
    ignoring_values_in_pattern();

    println!("");
    underscores();

    println!("");
    ignore_with_dots();

    println!("");
    match_guards();
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

fn multiple_pattern_matching() {
    println!("multiple_pattern_matching:");

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("all other numbers"),
    }
}

fn range_matching() {
    println!("range_matching:");

    let x = 5;

    match x {
        1..=5 => println!("one through 5"),
        _ => println!("something else"),
    }

    // range matching with chars
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn destructuring() {
    println!("destructuring:");

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // shorthand, can just use x,y:
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    // matching literals:
    match p {
        Point { x, y: 0 } => println!("on x axis at {x}"), // matches if y is 0
        Point { x: 0, y } => println!("on y axis at {y}"), // mathes if x is 0
        _ => println!("on neither axis: ({x}, {y})"), // matches if both not 0 (all other cases)
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_matching() {
    println!("enum_matching:");

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("Nothing to match on with Quit"),
        Message::Move { x, y } => println!("moved in x direction {x} and y direction {y}"),
        Message::Write(val) => println!("wrote {val}"),
        Message::ChangeColor(r, g, b) => println!("changed color to RGB({r},{g},{b})"),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NestedMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn nested_enum_matching() {
    println!("nested_enum_matching:");

    let msg = NestedMessage::ChangeColor(Color::Hsv(0, 160, 255));

    // if it's an RGB variant, goes in first arm
    // if it's an HSV variant, goes in second arm

    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("changed color to RGB({r},{g},{b})");
        }
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("changed color to HSV({h},{s},{v})");
        }
        _ => (),
    }
}

fn struct_tuple_matching() {
    println!("struct_tuple_matching:");

    // (tuple, struct)
    let stuff = ((3, 10), Point { x: 3, y: -10 });
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("{stuff:?}");
    println!("( feet: {feet}, inches: {inches} ), Point({x}, {y})");
}

fn foo(_: i32, y: i32) {
    println!("This code only uses param y: {y}");
}

fn ignoring_values_in_pattern() {
    println!("ignoring_values_in_pattern:");

    // ignore entire value with "_"
    foo(5, 6);

    // ignore parts of value with nested "_"
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // allows only updating setting value if not yet set, or to unset it:
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't override an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting: {setting_value:?}");

    // ignoring particular values
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers: {first}, {third}, {fifth}");
        }
    }
}

fn underscores() {
    println!("underscores:");

    let s = Some(String::from("Hello!"));

    // this will cause an error:
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    //
    // s moved into _s above
    // println!("{s:?}");

    // doesn't actually bind to the value
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");
}

struct MyPoint {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_with_dots() {
    println!("ignore_with_dots:");

    let origin = MyPoint { x: 0, y: 0, z: 0 };

    // use ".." to only match on x:
    match origin {
        MyPoint { x, .. } => println!("x is {x}"),
    }

    // ".." expands as far as it needs to:
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}"),
    }

    // ambiguous - won't work:
    // match numbers {
    //     (.., second, ..) => println!("Some numbers: {second}"),
    // }
}

fn match_guards() {
    println!("match_guards:");

    // additional if condition after pattern - used to determine branch execution as well
    // downside: compiler doesn't check for exhaustiveness

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("x is even: {x}"),
        Some(x) => println!("x is not even: {x}"),
        None => ()
    }
}
