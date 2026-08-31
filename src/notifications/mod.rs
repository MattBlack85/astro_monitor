pub mod lan;
pub mod telegram;

/// Common interface for all notification backends.
pub trait Notification {
    /// Send a notification with the given message.
    /// Returns Ok(()) on success, or an error description on failure.
    fn send(&self, message: &str) -> Result<(), String>;
}

/// Fans a notification out to several backends.
///
/// Succeeds as soon as one backend delivers. That is the whole point of
/// running Telegram and LAN together: at a dark site the Telegram relay is
/// unreachable, and the alert must still count as delivered because the LAN
/// listener received it.
pub struct MultiNotifier {
    backends: Vec<Box<dyn Notification>>,
}

impl MultiNotifier {
    pub fn new(backends: Vec<Box<dyn Notification>>) -> Self {
        Self { backends }
    }
}

impl Notification for MultiNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        if self.backends.is_empty() {
            return Err("no notification backend configured".to_string());
        }
        let mut errors: Vec<String> = Vec::new();
        for backend in &self.backends {
            match backend.send(message) {
                Ok(_) => return Ok(()),
                Err(e) => errors.push(e),
            }
        }
        Err(errors.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Stub {
        ok: bool,
        calls: Rc<RefCell<usize>>,
    }

    impl Stub {
        fn new(ok: bool, calls: Rc<RefCell<usize>>) -> Self {
            Self { ok, calls }
        }
    }

    impl Notification for Stub {
        fn send(&self, _message: &str) -> Result<(), String> {
            *self.calls.borrow_mut() += 1;
            if self.ok {
                Ok(())
            } else {
                Err("stub failure".to_string())
            }
        }
    }

    fn counter() -> Rc<RefCell<usize>> {
        Rc::new(RefCell::new(0))
    }

    #[test]
    fn succeeds_when_any_backend_delivers() {
        let (a, b) = (counter(), counter());
        let multi = MultiNotifier::new(vec![
            Box::new(Stub::new(false, Rc::clone(&a))),
            Box::new(Stub::new(true, Rc::clone(&b))),
        ]);
        assert!(multi.send("x").is_ok());
        // The first one failed, so the second must have been tried.
        assert_eq!(*a.borrow(), 1);
        assert_eq!(*b.borrow(), 1);
    }

    #[test]
    fn stops_at_the_first_success() {
        let (a, b) = (counter(), counter());
        let multi = MultiNotifier::new(vec![
            Box::new(Stub::new(true, Rc::clone(&a))),
            Box::new(Stub::new(true, Rc::clone(&b))),
        ]);
        assert!(multi.send("x").is_ok());
        assert_eq!(*a.borrow(), 1);
        assert_eq!(*b.borrow(), 0, "the second backend must not be tried");
    }

    #[test]
    fn fails_only_when_every_backend_fails() {
        let (a, b) = (counter(), counter());
        let multi = MultiNotifier::new(vec![
            Box::new(Stub::new(false, Rc::clone(&a))),
            Box::new(Stub::new(false, Rc::clone(&b))),
        ]);
        let err = multi.send("x").unwrap_err();
        assert!(err.contains("stub failure"));
        assert_eq!(*a.borrow(), 1);
        assert_eq!(*b.borrow(), 1);
    }

    #[test]
    fn empty_backend_list_is_an_error() {
        let multi = MultiNotifier::new(vec![]);
        assert!(multi.send("x").is_err());
    }
}
