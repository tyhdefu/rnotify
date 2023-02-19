use std::error::Error;
use std::fmt::Debug;
use notify_rust::{Hint, Notification, Urgency};
use serde::{Serialize, Deserialize};
use crate::destination::{MessageDestination, SerializableDestination};
use crate::message::{Level, Message, MessageDetail};

/// Receive a notification to the desktop
///
/// # KDE #
/// You need to enable "Other applications" notifications to show in tray
/// if you want these to persist - at least when running with `cargo run` / without
/// an installation.
#[derive(Debug, Serialize, Deserialize)]
pub struct DesktopNotificationReceiver {}

impl MessageDestination for DesktopNotificationReceiver {
    fn send(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        let mut notification = Notification::new();
        let mut title = String::new();
        if let Some(message_title) = message.get_title() {
            title.push_str(message_title)
        }
        if let Some(component) = message.get_component() {
            if !title.is_empty() {
                title.push_str(" - ");
            }
            title.push_str(&format!("{}", component));
        }
        notification.summary(&title)
            .auto_icon();

        let body = format!("{}\nFrom: {}", message.get_message_detail().raw(), message.get_author());

        notification.body(&body);

        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let urgency = match message.get_level() {
                Level::Info => Urgency::Normal,
                Level::Warn => Urgency::Normal,
                Level::Error => Urgency::Critical,
                Level::SelfError => Urgency::Critical,
            };

            notification.urgency(urgency);


            // TODO: Can we set icon on macos / windows
            // https://wiki.archlinux.org/title/Desktop_notifications#Bash
            // https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
            let icon = match message.get_level() {
                Level::Info =>      "dialog-information",
                Level::Warn =>      "dialog-warning",
                Level::Error =>     "dialog-error",
                Level::SelfError => "dialog-error",
            };
            notification.icon(icon);
        }

        notification.show()?;

        Ok(())
    }
}


#[typetag::serde(name = "Desktop")]
impl SerializableDestination for DesktopNotificationReceiver {
    fn as_message_destination(&self) -> &dyn MessageDestination {
        self
    }
}