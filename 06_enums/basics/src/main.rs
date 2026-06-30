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
    // left off here: https://doc.rust-lang.org/book/ch06-02-match.html#patterns-that-bind-to-values
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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn match_with_enum() {
    let my_penny: Coin = Coin::Penny;

    println!("my pennie's value: {}", value_in_cents(&my_penny));

    let other_val = value_in_cents(&my_penny);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
