use std::fmt;

enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    putting_data_inside_enums();

    // calling method on enum
    let write_message = Message::Write(String::from("hello"));

    write_message.call();

    // using the move enum (struct-like type)
    let my_move = Message::Move { x: 50, y: -50 };

    my_move.call();

    option_enum();
    match_with_enum();
    option_matching();
    catch_all_patterns();
    if_let_basics();
    if_let_else();
    let_else();
}

// can use any V4 or V6
fn route(ip: IpAddrKind) {}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn putting_data_inside_enums() {
    let home = IpAddr::V4(127, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Message enum as defined with equivalent types using structs:
struct QuitMessage; // unit struct (empty)
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColor(i32, i32, i32); // tuple struct

// can add methods on enums
impl Message {
    fn call(&self) {
        // do something
    }
}

fn option_enum() {
    // examples
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // handle optional y
    let sum = match y {
        Some(y) => y + x,
        None => x,
    };

    println!("sum: {sum}");
}

#[derive(Debug)] // so we can inspect the state
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_with_enum() {
    let my_penny: Coin = Coin::Penny;
    println!("my pennie's value: {}", value_in_cents(&my_penny));

    let alaska_quarter: Coin = Coin::Quarter(UsState::Alaska);
    println!("alaska quarter value: {}", value_in_cents(&alaska_quarter));
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn option_matching() {
    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(six);

    println!("six is {}", six.unwrap());
    println!("seven is {}", seven.unwrap());

    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all_patterns() {
    let dice_roll = 9;

    // catch all
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catches all other cases
    }

    // can use "_" if don't need catch-all var
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll(), // catches all other cases
    }

    // can use unit "()" if you want nothing to happen on all other cases
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // catches all other cases, does nothing
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(moves: u8) {}
fn re_roll() {}

fn if_let_basics() {
    let config_max = Some(3u8);

    // only execute code if max is "Some" with match (little too verbose)
    match config_max {
        Some(max) => println!("Max is {max}"),
        _ => (),
    }

    // can instead use if let:
    if let Some(max) = config_max {
        println!("Max is {max}");
    }

    // Use if let when you only want to do something for one case (don't need exhaustive cases that
    // match offers)
}

fn if_let_else() {
    // Imagine you want to call out the quarter's state if it's a quarter, else increment count of
    // all other coins.
    // You can do this with a match as seen below.
    // Or you can also do this with an "if let else" below.

    let coin: Coin = Coin::Quarter(UsState::Alaska);
    let other_coin: Coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;

    // using match
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // using if let else
    if let Coin::Quarter(state) = other_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}

fn let_else() {
    println!("let_else");

    let alaska_quarter = Coin::Quarter(UsState::Alaska);

    let description = describe_state_quarter_three(alaska_quarter);

    if let Some(val) = description {
        println!("description: {val}");
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter_one(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }

    } else {
        None
    }
}

fn describe_state_quarter_two(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// with "let...else" syntax
fn describe_state_quarter_three(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
