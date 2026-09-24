pub trait Messenger {
    // takes immutable ref to self & msg
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T, // instance won't outlive this ref. ref will be available
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where 
    T: Messenger,
{
    // LimitTracker has a new method to create a new instance
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over the quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Warning: You've used up over 90% of your quota.");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota.");
        }
    }
}

// Mock object that just keeps track of messages its told to send
// to test the whole functionality - should send appropriate message

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![]
            }
        }
    }

    // Implement the Messenger trait
    impl Messenger for MockMessenger {

        // "fake" send to keep capture messages to assert on
        fn send(&self, msg: &str) {
            self.sent_messages.push(msg.to_string());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 100);

        tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }

    // TODO: left off https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#listing-15-21
}
