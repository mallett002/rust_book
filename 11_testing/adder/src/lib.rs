/*
cargo test [cargo flags] -- [test binary flags]

ex: `cargo test --release foo -- --test-threads=1`

breakdown:
 --release → Cargo builds in release mode
 foo → Cargo filters to tests containing "foo"
 -- → separator
 --test-threads=1 → test binary runs one test at a time

*/

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn add_two(n: u64) -> u64 {
    n + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

// needs to implement Debug & PartialEq for assert_eq!
#[derive(Debug)]
#[derive(PartialEq)] // 
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new_with_height(height: u32) -> Self {
        Self {
            width: 30,
            height,
        }
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_two() {
        let result = add_two(5);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_new_with_height() {
        let height = 40;
        let result = Rectangle::new_with_height(height);
        let expected = Rectangle {
            width: 30,
            height,
        };

        // requires Debug & PartialEq for assert_eq!
        assert_eq!(result, expected);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`", 
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // matches substr of panic msg
    fn greater_than_100() {
        Guess::new(101);
    }

    // Can also use Result<T, E> on tests
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 did not equal 4"))
        }
    }

    // Src code prints_and_returns_10 has println
    // Passing test hides std output (printlns) by default
    // `cargo test -- --show-output` to show them
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    // see output here bc test fails
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    // TODO: left off https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-a-subset-of-tests-by-name
}
