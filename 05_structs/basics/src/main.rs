struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    // create a "User" instance
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 100,
    };

    // mutable User
    let mut michael = User {
        active: false,
        username: String::from("mscott123"),
        email: String::from("mscott123@dundermifflen.com"),
        sign_in_count: 1,
    };

    // update values on mutable struct
    michael.active = true;


    // call builder func that builds a struct
    let dwight_username = String::from("schruted");
    let dwight_email = String::from("schruted@dundermifflen.com");

    let dwight = build_user(
        dwight_username,        
        dwight_email,
    );

    // left off: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}
