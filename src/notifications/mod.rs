pub mod telegram;

/// Common interface for all notification backends.
pub trait Notification {
    /// Send a notification with the given message.
    /// Returns Ok(()) on success, or an error description on failure.
    fn send(&self, message: &str) -> Result<(), String>;
}
