use error::*;
use libnotify;

pub use libnotify::Urgency;

const ERROR_NAME: &str = "libnotify";

#[derive(Debug)]
pub struct LibNotify {}

impl LibNotify {
    pub fn new() -> Result<Self> {
        libnotify::init("dwm-status").wrap_error(ERROR_NAME, "init failed")?;

        Ok(LibNotify {})
    }

    pub fn send_notification(&self, summary: &str, body: &str, urgency: Urgency) -> Result<()> {
        let notification = libnotify::Notification::new(summary, Some(body), None);
        notification.set_urgency(urgency);
        notification
            .show()
            .wrap_error(ERROR_NAME, "send notification failed")
    }
}

impl Drop for LibNotify {
    fn drop(&mut self) {
        libnotify::uninit();
    }
}
