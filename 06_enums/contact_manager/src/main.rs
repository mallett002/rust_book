use std::io;

fn main() {
    println!("=== Contact Manager ===");
    println!(
        "1. Add contact\n
2. List contacts\n
3. Find contact\n
4. Update contact\n
5. Delete contact\n
6. Quit\n
"
    );

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("expected an option");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("choose valid option: 1-6");
                continue;
            }
        };

        match choice {
            1 => add_contact(),
            2 => list_contacts(),
            3 => find_contact(),
            4 => update_contact(),
            5 => delete_contact(),
            6 => quit(),
            _ => println!("choose valid option: 1-6"),
        }

        if choice == 6 {
            println!("exiting...");
            break;
        }
    }
}

fn add_contact() {
    println!("add_contact");
}
fn list_contacts() {
    println!("list_contacts");
}
fn find_contact() {
    println!("find_contact");
}
fn update_contact() {
    println!("update_contact");
}
fn delete_contact() {
    println!("delete_contact");
}
fn quit() {
    println!("quit");
}
