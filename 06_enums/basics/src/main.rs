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
