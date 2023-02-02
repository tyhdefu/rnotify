use std::error::Error;
use crate::message::Message;
use crate::send_error::SendError;

#[derive(Debug)]
pub struct SendErrorBorrowed<'a> {
    err: Box<dyn Error>,
    destination_id: String,
    message: &'a Message,
}

impl<'a> SendErrorBorrowed<'a> {
    pub fn create(err: Box<dyn Error>, item_id: String, message: &'a Message) -> Self {
        Self {
            err,
            destination_id: item_id,
            message,
        }
    }
}

impl<'a> SendError for SendErrorBorrowed<'a> {
    fn get_error(&self) -> &Box<dyn Error> {
        &self.err
    }

    /// The id of the destination that a message could not be sent to
    fn get_failed_destination_id(&self) -> &str {
        &self.destination_id
    }

    /// The message that was not sent
    fn get_failed_message(&self) -> &Message {
        self.message
    }
}