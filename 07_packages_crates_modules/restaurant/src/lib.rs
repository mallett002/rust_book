// root crate and module tree:

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // calling super functions
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // call parent (global) fn
    }

    fn cook_order() {}

    // structs and enums public
    // public struct with only 1 member public. fruit isn't allowed to be altered.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // public enum, all fields are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house;
    
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_wait_list();

        // Relative path
        hosting::add_to_wait_list(); // just bring hosting into scope, not add_to_wait_list

        // order breakfast with rye bread
        let mut meal = back_of_house::Breakfast::summer("Rye");

        // change our mind about what type of bread
        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        // Following line won't compile. It's a private field
        // meal.seasonal_fruit = String::from("Blueberries");

        // using public enum:
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}

fn deliver_order() {}

