use csv::Reader;
use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug)]
enum ContactMethod {
    Phone(String),
    Email(String),
}

#[derive(Debug)]
struct Contact {
    first: String,
    last: String,
    contact_methods: Vec<ContactMethod>,
}

fn main() {
    let contacts = read_contacts().unwrap_or_default();

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
            1 => add_contact(&contacts),
            2 => list_contacts(&contacts),
            3 => find_contact(),
            4 => update_contact(),
            5 => delete_contact(),
            6 => (),
            _ => println!("choose valid option: 1-6"),
        }

        if choice == 6 {
            println!("Thank you. Please come again!");
            break;
        }
    }
}

fn prompt_menu() {
    println!("\n=== Contact Manager ===");
    println!("Select option (ex: 1 to add contact)");
    println!(
        "1. Add contact\n2. List contacts\n3. Find contact\n4. Update contact\n5. Delete contact\n6. Quit\n"
    );
}

fn read_contacts() -> Result<Vec<Contact>, Box<dyn Error>> {
    let mut rdr = Reader::from_path("contacts.csv")?;

    let mut contacts = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let first = record.get(0).unwrap_or("").to_string();
        let last = record.get(1).unwrap_or("").to_string();

        // build up contact_methods
        let mut contact_methods = Vec::new();

        let methods_str = record.get(2).unwrap_or("");

        if !methods_str.is_empty() {
            for method in methods_str.split("|") {
                let mut kv = method.splitn(2, ':');

                let tag = kv.next().unwrap_or("");
                let val = kv.next().unwrap_or("").to_string();

                match tag {
                    "Phone" => contact_methods.push(ContactMethod::Phone(val)),
                    "Email" => contact_methods.push(ContactMethod::Email(val)),
                    _ => {}
                }
            }
        }

        contacts.push(Contact {
            first,
            last,
            contact_methods,
        });
    }

    Ok(contacts)
}


fn add_contact(current_contacts: &Vec<Contact>) {
    // ----------- first name -------------------------
    println!("Enter first name:");

    let first = loop {
        let mut input = String::new();

        // using match here instead of .expect()
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => panic!("expected a first name: {e}"),
        }

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
        println!(
            "Select option:\n '1': phone\n '2': email\n 'done': finish contact methods\n 'cancel': cancel"
        );

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("expected an option: 1, 2 or done");

        let input = input.trim().to_string();

        if input == "cancel" {
            break;
        }

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
            } else {
                break;
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
            } else {
                break;
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

    let contact = Contact {
        first,
        last,
        contact_methods,
    };

    println!("\ncontact entered: {:#?}\n", contact);

    add_contact_to_all_contacts(&contact, current_contacts)
}

fn list_contacts(current_contacts: &Vec<Contact>) {
    // build up a string
    // loop over each contact and make it pretty for the print
    // print
    let mut listed_contacts = String::new();

    for contact in current_contacts {
        let methods: Vec<String> = contact
            .contact_methods
            .iter()
            .map(|m| match m {
                ContactMethod::Phone(p) => format!("Phone:{p}"),
                ContactMethod::Email(e) => format!("Email:{e}"),
            })
            .collect();

        let methods_str = methods.join("|");

        listed_contacts.push_str(&format!(
            "{},{},{}\n",
            contact.first, contact.last, methods_str
        ));
    }

    println!("{}", listed_contacts);
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

fn add_contact_to_all_contacts(new_contact: &Contact, current_contacts: &Vec<Contact>) {
    let mut csv = String::new();

    // 1. add header
    csv.push_str(&format!(
        "{},{},{}\n",
        "first_name", "last_name", "contact_methods"
    ));

    // 2. build up current contacts to write to csv
    for contact in current_contacts {
        let methods: Vec<String> = contact
            .contact_methods
            .iter()
            .map(|m| match m {
                ContactMethod::Phone(p) => format!("Phone:{p}"),
                ContactMethod::Email(e) => format!("Email:{e}"),
            })
            .collect();

        let methods_str = methods.join("|");

        csv.push_str(&format!(
            "{},{},{}\n",
            contact.first, contact.last, methods_str
        ));
    }

    // 3. add new contact in
    let methods: Vec<String> = new_contact
        .contact_methods
        .iter()
        .map(|m| match m {
            ContactMethod::Phone(p) => format!("Phone:{p}"),
            ContactMethod::Email(e) => format!("Email:{e}"),
        })
        .collect();

    let methods_str = methods.join("|");

    csv.push_str(&format!(
        "{},{},{}\n",
        new_contact.first, new_contact.last, methods_str
    ));

    println!("csv: {}", csv);

    fs::write("contacts.csv", csv).expect("failed to write contacts");
}

