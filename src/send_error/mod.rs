use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::message::author::Author;
use crate::message::{Level, Message, MessageDetail};
use crate::send_error::reported::ReportedSendError;

pub mod borrowed;
pub mod owned;
pub mod reported;

pub trait SendError {
    fn get_error(&self) -> &Box<dyn Error>;

    fn get_failed_destination_id(&self) -> &str;

    fn get_failed_message(&self) -> &Message;

    fn create_report_message(&self) -> Message {
        Message::new(Level::SelfError,
                     Some(format!("Failed to send notification to destination {}", self.get_failed_destination_id())),
                     MessageDetail::Raw(format!("Rnotify failed to send a message {:?} to destination id '{}'. Error: '{}' A notification has been sent here because this is configured as a root logger.",
                                                         self.get_failed_message(), self.get_failed_destination_id(), self.get_error())),
                     None,
                     Author::parse("rnotify".to_owned()),
                     self.get_failed_message().get_unix_timestamp_millis().clone(),
        )
    }
}

/// A struct containing information about what parts succeeded and what parts
/// failed about [MessageRouter::route](crate::message_router::MessageRouter::route) attempt.
#[derive(Debug)]
pub struct SendErrors<'a> {
    successfully_sent: usize,
    original_message: &'a Message,
    errors: Vec<ReportedSendError<'a>>,
}

impl<'a> SendErrors<'a> {

    pub fn new(message: &'a Message, errors: Vec<ReportedSendError<'a>>, successfully_sent: usize) -> Self {
        Self {
            successfully_sent,
            original_message: message,
            errors,
        }
    }

    /// Get the message that caused errors when sent.
    pub fn get_message(&self) -> &Message {
        self.original_message
    }
}

impl<'a> Display for SendErrors<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-----")?;
        writeln!(f, "Summary:")?;
        writeln!(f, "Successfully sent to {} destinations", self.successfully_sent)?;
        writeln!(f, "Failed to send to {} destinations", self.errors.len())?;
        writeln!(f, "Message: {:?}", self.original_message)?;

        for error in &self.errors {
            writeln!(f, "--")?;
            writeln!(f, "Failed to send a message to destination '{:?}'", error.get_failed_destination_id())?;
            writeln!(f, "Due to error: {:?}", error.get_error())?;

            let any_root_fails = error.get_report_summary().get_report_failures().is_empty();

            if error.get_report_summary().was_reported() {
                writeln!(f, "This was reported to atleast one destination.")?;
            }
            else if !any_root_fails {
                // Not reported and no fails to report -> nowhere was applicable
                writeln!(f, "No error receiving destinations were applicable to send this error to.")?;
            }

            if any_root_fails {
                writeln!(f, "Some Error receiving destinations failed to be reported to.")?;

                for err in error.get_report_summary().get_report_failures() {
                    writeln!(f, "   - {:?}: {:?} ; Tried to send: {:?}", err.get_failed_destination_id(), err.get_error(), err.get_failed_message())?;
                }
            }
        }
        writeln!(f, "-----")
    }
}
