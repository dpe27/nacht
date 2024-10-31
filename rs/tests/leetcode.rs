trait Messenger {
    fn send(&self, message: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Unrgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod leetcode_test {

    use std::{cell::RefCell, vec};

    use super::*;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }   
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_mut_borrow = self.send_messages.borrow_mut();
            let mut two_mut_borrow = self.send_messages.borrow_mut();

            one_mut_borrow.push(String::from(message));
            two_mut_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_send_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}
