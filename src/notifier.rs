use mac_notification_sys::error::Error;
use mac_notification_sys::{Notification, send_notification};

pub struct Notifier {}

impl Notifier {
    pub fn new() -> Notifier {
        Notifier {}
    }

    pub fn send_notification(&self) -> Result<(), Error> {
        send_notification("Timer completed",
                          None,
                          "Timer completed",
                          Some(Notification::new().sound("Blow")))?;
        Ok(())
    }
}