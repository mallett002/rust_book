use std::io;

#[derive(Debug)]
enum ContactMethod {
    Phone(String),
    Email(String),
}

struct Contact {
    first: String,
    last: String,
    contact_methods: Vec<ContactMethod>,
}

fn prompt_menu() {
    println!("\n=== Contact Manager ===");
    println!("Select option (ex: 1 to add contact)");
    println!("1. Add contact\n2. List contacts\n3. Find contact\n4. Update contact\n5. Delete contact\n6. Quit\n");
}

fn main() {
    loop {
        prompt_menu();

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
    // ----------- first name -------------------------
    println!("Enter first name:");

    let first = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("expected a first name");
        let input = input.trim().to_string();

        if !input.is_empty() {
            break input;
        }

        println!("name cannot be empty, try again.");
    };

    // ----------- last name -------------------------
    println!("Enter last name:");

    let last = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("expected a last name");
        let input = input.trim().to_string();

        if !input.is_empty() {
            break input;
        }

        println!("last name cannot be empty, try again.");
    };

    // ----------- contact methods -------------------------
    println!("Enter contact methods");

    let mut contact_methods: Vec<ContactMethod> = Vec::new();

    loop {
        println!("Enter 1 for phone, 2 for email, or enter 'done' to finish contact methods:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("expected an option: 1, 2 or done");

        let input = input.trim().to_string();

        if input == "1" {
            println!("Enter phone, or enter 'cancel' to cancel:");

            let phone = loop {
                let mut input = String::new();

                io::stdin().read_line(&mut input).expect("expected a phone");
                let input = input.trim().to_string();

                if !input.is_empty() {
                    break input;
                }

                println!("phone cannot be empty, try again.");
            };

            if phone != "cancel" {
                let phone = ContactMethod::Phone(phone);
                contact_methods.push(phone);
            }
        }

        if input == "2" {
            println!("Enter email, or enter 'cancel' to cancel:");

            let email = loop {
                let mut input = String::new();

                io::stdin().read_line(&mut input).expect("expected a email");
                let input = input.trim().to_string();

                if !input.is_empty() {
                    break input;
                }

                println!("email cannot be empty, try again.");
            };

            if email != "cancel" {
                let email = ContactMethod::Email(email);
                contact_methods.push(email);
            }
        }

        if input == "done" {
            if contact_methods.is_empty() {
                println!("contact methods cannot be empty, try again.");
            } else {
                break;
            }
        }
    }

    println!("\ncontacts entered: {:?}\n", contact_methods);
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
